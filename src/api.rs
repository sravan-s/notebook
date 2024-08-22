use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use axum::{
    extract::{self, Path},
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::{app_state, models::notebook};

pub async fn start(app_state: app_state::AppState) -> Result<()> {
    let app_state = Arc::from(app_state);
    println!("Setting up API server");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/", get(|| async { "Server running" }))
        .route(
            "/notebooks",
            get({
                let app_state = Arc::clone(&app_state);
                move || notebook::api_handlers::get_notebooks(app_state)
            }),
        )
        .route(
            "/notebook",
            post({
                let app_state = Arc::clone(&app_state);
                move |extract::Json(payload): extract::Json<
                    notebook::model::CreateNotebookPayload,
                >| { notebook::api_handlers::create_notebook(payload, app_state) }
            }),
        )
        .route(
            "/notebook/:id",
            get({
                let app_state = Arc::clone(&app_state);
                move |Path(notebook_id): Path<String>| {
                    notebook::api_handlers::get_notebook_with_id(notebook_id, app_state)
                }
            })
            /* we are not doing this, not worth at this state
            .put({
                let app_state = Arc::clone(&app_state);
                move |Path(notebook_id): Path<String>| {
                    notebook::api_handlers::get_notebook_with_id(notebook_id, app_state)
                }
            }) */
            .delete({
                let app_state = Arc::clone(&app_state);
                move |Path(notebook_id): Path<String>| {
                    notebook::api_handlers::delete_notebook_with_id(notebook_id, app_state)
                }
            }),
        )
        /*
        .route("/notebook_versions", get())
        .route("/notebook_version/:id", get().post().delete()) // no
        // updates
        .route("/notebook_version_paragraphs/:id", get())
        .route("/notebook_paragraphs/:id", get())
        .route("/paragraph/:id", get().post().put().delete())
        .route("/run/paragraph/:id", post())
        .route("/run/notebook_version/:id", post())
        .route("/run/notebook/:id", post())
        */
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
