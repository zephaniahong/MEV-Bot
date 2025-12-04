use alloy::{
    consensus::Transaction,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use anyhow::Result;
use futures_util::StreamExt;

use crate::constants::UNISWAP_V2_ROUTER;

pub async fn start_ingestor() -> Result<()> {
    let ws_url = std::env::var("WS_URL").unwrap();
    let ws = WsConnect::new(ws_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    let sub = provider.subscribe_pending_transactions();
    let stream = sub.await?.into_stream();

    let mut buffered_stream = stream
        .map(
            async |tx_hash| match provider.get_transaction_by_hash(tx_hash).await {
                Ok(Some(tx)) if tx.to() == Some(UNISWAP_V2_ROUTER) => Some(tx),
                _ => None,
            },
        )
        .buffer_unordered(10)
        .filter_map(|res| std::future::ready(res));

    while let Some(tx) = buffered_stream.next().await {
        println!("TX: {:?}", tx);
    }

    Ok(())
}
