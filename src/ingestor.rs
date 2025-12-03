use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use anyhow::Result;
use futures_util::StreamExt;

pub async fn start_ingestor() -> Result<()> {
    let ws_url = std::env::var("WS_URL").unwrap();
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let sub = provider.subscribe_pending_transactions();
    let mut stream = sub.await?.into_stream();

    while let Some(tx_hash) = stream.next().await {
        println!("Block: {}", tx_hash);
    }

    Ok(())
}
