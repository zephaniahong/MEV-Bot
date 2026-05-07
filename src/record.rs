use std::time::{SystemTime, UNIX_EPOCH};

use alloy::rpc::types::{Log, Transaction};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncWriteExt, BufWriter},
};

#[derive(Serialize, Deserialize)]
pub struct LogEvent {
    ts: u128,
    tx: Transaction,
}

impl LogEvent {
    pub fn new(tx: &Transaction) -> Self {
        Self {
            ts: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros(),
            tx: tx.clone(),
        }
    }
}

pub struct Recorder {
    writer: BufWriter<File>,
}

impl Recorder {
    pub async fn new(path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await
            .unwrap();

        Self {
            writer: BufWriter::new(file),
        }
    }

    pub async fn record<T: Serialize>(&mut self, event: &T) {
        let val_str = serde_json::to_string(event).unwrap();
        let val_bytes = val_str.as_bytes();
        self.writer.write_all(val_bytes).await.unwrap();
        self.writer.write_all(b"\n").await.unwrap();
    }
}
