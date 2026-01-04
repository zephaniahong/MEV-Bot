use futures_util::StreamExt;
use std::{collections::HashMap, ops::Add, sync::Arc};
use tokio::sync::RwLock;

use alloy::{
    primitives::{Address, U256, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol_types::SolEvent,
};
use anyhow::Result;
use tracing::{error, info};

use crate::{
    types::{IUniswapV2Pair, Sync},
    utils::calculate_pair_address,
};

mod constants;
mod sniper;
mod types;
mod utils;

async fn state_updater<P>(
    provider: P,
    cache: Arc<RwLock<HashMap<Address, Pool>>>,
    pool: Pool,
) -> Result<()>
where
    P: Clone + Send + core::marker::Sync + 'static,
    P: Provider,
{
    let latest_block = provider.get_block_number().await?;
    let filter = Filter::new()
        .event(Sync::SIGNATURE)
        .address(pool.address)
        .from_block(latest_block);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let pool_clone = pool.clone();
        match log.log_decode::<Sync>() {
            Ok(decoded) => {
                let data = decoded.data();
                let mut guard = cache.write().await;
                if let Some(val) = guard.get_mut(&pool_clone.address) {
                    val.reserve0 = U256::from(data.reserve0);
                    val.reserve1 = U256::from(data.reserve1);
                } else {
                    guard.insert(pool.address, pool_clone);
                }
                info!("Inserted {:?}", pool);
            }
            Err(e) => error!("Decode error: {e:?}, raw log: {log:?}"),
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Pool {
    address: Address,
    token0: Address,
    token1: Address,
    reserve0: U256,
    reserve1: U256,
}

impl Pool {
    pub fn new(
        address: Address,
        token0: Address,
        token1: Address,
        reserve0: U256,
        reserve1: U256,
    ) -> Self {
        Self {
            address,
            token0,
            token1,
            reserve0,
            reserve1,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting MEV Engine");

    let ws_url = std::env::var("WS_URL").unwrap();
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;
    let cache = Arc::new(RwLock::new(HashMap::new()));

    {
        let provider_clone = provider.clone();
        let cache_clone = cache.clone();
        let usdc_weth_address = address!("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc");
        let add = calculate_pair_address(
            address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            address!("0x2581ceba70876dae5c8b00472f9633ef1428baa1"),
        );
        let pair = IUniswapV2Pair::new(add, provider_clone.clone());
        let token0 = Address::from(pair.token0().call().await?.0);
        let token1 = Address::from(pair.token1().call().await?.0);
        let reserve0 = U256::from(pair.getReserves().call().await?.reserve0);
        let reserve1 = U256::from(pair.getReserves().call().await?.reserve1);
        let pool = Pool::new(add, token0, token1, reserve0, reserve1);
        tokio::spawn(async move {
            if let Err(e) = state_updater(provider_clone, cache_clone, pool).await {
                error!("Error updating state: {e}");
            }
        });
    }

    sniper::start_sniper(provider, cache).await
}
