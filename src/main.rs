use anyhow::{Context, Result};

mod bootstrap;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap::run()
        .await
        .context("Bootstrapping application failed")
        .unwrap();
    Ok(())
}
