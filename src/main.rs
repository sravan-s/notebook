use anyhow::{Context, Result};
use app_state::AppState;

mod api;
mod app_state;
mod bootstrap;
mod db;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap::run()
        .await
        .context("Bootstrapping application failed")
        .unwrap();
    let app_state = AppState::new().await;
    api::start(app_state)
        .await
        .context("Setup API server")
        .unwrap();
    Ok(())
}
