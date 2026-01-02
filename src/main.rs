use anyhow::Result;
use tracing::info;

mod constants;
mod ingestor;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    info!("Starting MEV Engine");

    ingestor::start_ingestor().await
}
