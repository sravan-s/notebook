use anyhow::{Context, Result};

mod api;
mod bootstrap;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap::run()
        .await
        .context("Bootstrapping application failed")
        .unwrap();
    api::start().await.context("Setup API server").unwrap();
    Ok(())
}
