use futures_util::StreamExt;
use std::sync::RwLock;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use alloy::{
    primitives::{Address, U256, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol_types::SolEvent,
};
use anyhow::{Context, Result};
use tracing::{error, info};

use crate::{
    record::LogEvent,
    types::{IUniswapV2Pair, Sync},
    utils::calculate_pair_address,
};

mod constants;
mod record;
mod sniper;
mod strategy;
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
                let mut guard = cache.write().unwrap();
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

fn load_events(path: &str) -> Vec<LogEvent> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut events = Vec::with_capacity(300_000);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let event: LogEvent = serde_json::from_str(&line).expect("Failed to parse JSON");
        events.push(event);
    }
    events
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting MEV Engine");

    let ws_url = std::env::var("WS_URL").context("WS_URL must be set")?;
    info!("Connecting to WebSocket provider");
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new()
        .connect_ws(ws)
        .await
        .context("failed to connect to WebSocket provider")?;
    info!("Connected to WebSocket provider");
    let cache = Arc::new(RwLock::new(HashMap::new()));

    {
        let provider_clone = provider.clone();
        let cache_clone = cache.clone();
        let usdc_weth_address = address!("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc");
        let add = calculate_pair_address(
            address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"), // WETH
            address!("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"), // USDC
        );
        info!("Loading initial pool state for {add}");
        let pair = IUniswapV2Pair::new(add, provider_clone.clone());
        let token0 = Address::from(pair.token0().call().await?.0);
        let token1 = Address::from(pair.token1().call().await?.0);
        let reserve0 = U256::from(pair.getReserves().call().await?.reserve0);
        let reserve1 = U256::from(pair.getReserves().call().await?.reserve1);
        let pool = Pool::new(add, token0, token1, reserve0, reserve1);
        info!("Loaded initial pool state: {pool:?}");
        tokio::spawn(async move {
            if let Err(e) = state_updater(provider_clone, cache_clone, pool).await {
                error!("Error updating state: {e}");
            }
        });
        info!("Started pool state updater");
    }

    info!("Starting pending transaction sniper");
    sniper::start_sniper(provider, cache).await
}
