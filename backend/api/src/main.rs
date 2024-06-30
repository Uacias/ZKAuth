mod args;
mod handlers;
mod models;
mod utils;

use args::Args;
use axum::{routing::post, Router};
use clap::Parser;
use handlers::auth::{register_no_hashing, AppState};
use std::sync::Arc;
use tracing::info;
use utils::setup_surrealdb::setup_surrealdb;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    let db = setup_surrealdb(args).await;

    let shared_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/register-no-hash", post(register_no_hashing))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
