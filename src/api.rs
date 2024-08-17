use anyhow::{Context, Ok, Result};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

pub async fn start() -> Result<()> {
    println!("Setting up API server");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/", get(|| async { "Server running" }))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .context("Listening to port 8080")
        .unwrap();
    axum::serve(listener, app)
        .await
        .context("Setting up API server")
        .unwrap();
    println!("Setup API");
    Ok(())
}
