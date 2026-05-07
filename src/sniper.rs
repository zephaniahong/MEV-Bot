use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::sync::Mutex;

use alloy::{primitives::Address, providers::Provider};
use anyhow::Result;
use futures_util::StreamExt;
use tracing::{debug, info, warn};

use crate::{
    Pool,
    record::{LogEvent, Recorder},
    strategy::Strategy,
};

/// Listens for pending tx and determines if there is a profitable opportunity
pub async fn start_sniper<P>(provider: P, cache: Arc<RwLock<HashMap<Address, Pool>>>) -> Result<()>
where
    P: Clone + Send + Sync + 'static,
    P: Provider,
{
    let strategy = Strategy::new(cache);
    let recorder = Arc::new(Mutex::new(Recorder::new("pending_transactions.json").await));
    let sub = provider.subscribe_pending_transactions().await?;
    let stream = sub.into_stream();
    info!("Subscribed to pending transaction stream");

    let mut buffered_stream = stream
        .map(|tx_hash| {
            let provider = provider.clone();
            let recorder = recorder.clone();
            async move {
                match provider.get_transaction_by_hash(tx_hash).await {
                    // Ok(Some(tx)) if tx.to() == Some(UNISWAP_V2_ROUTER) => Some(tx),
                    Ok(Some(tx)) => {
                        let event = LogEvent::new(&tx);
                        recorder.lock().await.record(&event).await;
                        Some(tx)
                    }
                    Ok(None) => {
                        debug!("Pending transaction {tx_hash} was not available yet");
                        None
                    }
                    Err(e) => {
                        warn!("Failed to fetch pending transaction {tx_hash}: {e}");
                        None
                    }
                }
            }
        })
        .buffer_unordered(10)
        .filter_map(|res| std::future::ready(res));

    let mut processed = 0_u64;
    while let Some(tx) = buffered_stream.next().await {
        processed += 1;
        if processed == 1 || processed % 100 == 0 {
            info!("Processed {processed} pending transactions");
        }
        strategy.process(&tx);
    }
    Ok(())
}
