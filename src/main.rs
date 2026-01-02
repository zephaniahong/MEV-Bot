use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use anyhow::Result;
use tracing::{error, info};

mod constants;
mod sniper;
mod types;
mod utils;

async fn state_updater<P>(
    provider: P,
    cache: Arc<RwLock<HashMap<Address, (u128, u128)>>>,
) -> Result<()>
where
    P: Clone + Send + Sync + 'static,
    P: Provider,
{
    todo!()
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    info!("Starting MEV Engine");

    let ws_url = std::env::var("WS_URL").unwrap();
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;
    let cache = Arc::new(RwLock::new(HashMap::new()));

    {
        let provider_clone = provider.clone();
        let cache_clone = cache.clone();
        tokio::spawn(async {
            if let Err(e) = state_updater(provider_clone, cache_clone).await {
                error!("Error updating state: {e}");
            }
        });
    }

    sniper::start_sniper(provider, cache).await
}
