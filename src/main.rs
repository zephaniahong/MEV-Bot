use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use alloy::{
    primitives::{Address, Uint, address},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol_types::SolEvent,
};
use anyhow::Result;
use tracing::{error, info};

use crate::types::Sync;

mod constants;
mod sniper;
mod types;
mod utils;

async fn state_updater<P>(
    provider: P,
    cache: Arc<RwLock<HashMap<Address, (Uint<112, 2>, Uint<112, 2>)>>>,
    pair_address: Address,
) -> Result<()>
where
    P: Clone + Send + core::marker::Sync + 'static,
    P: Provider,
{
    let latest_block = provider.get_block_number().await?;
    let filter = Filter::new()
        .event(Sync::SIGNATURE)
        .address(pair_address)
        .from_block(latest_block);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        match log.log_decode::<Sync>() {
            Ok(decoded) => {
                let data = decoded.data();
                let mut guard = cache.write().await;
                guard.insert(pair_address, (data.reserve0, data.reserve1));
                info!(
                    "Inserted {:?} for {}",
                    (data.reserve0, data.reserve1),
                    pair_address
                );
            }
            Err(e) => error!("Decode error: {e:?}, raw log: {log:?}"),
        }
    }

    Ok(())
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
        tokio::spawn(async move {
            if let Err(e) = state_updater(provider_clone, cache_clone, usdc_weth_address).await {
                error!("Error updating state: {e}");
            }
        });
    }

    sniper::start_sniper(provider, cache).await
}
