mod args;
use args::Args;

use axum::{extract::State, routing::post, Json, Router};
use clap::Parser;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserData {
    login: String,
    password: String,
}

#[derive(Clone, Debug)]
struct AppState {
    db: Surreal<Client>,
}

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

async fn register_no_hashing(
    State(app_state): State<Arc<AppState>>,
    Json(register_data): Json<RegisterUserData>,
) -> Result<(StatusCode, Json<Vec<RegisterUserData>>), (StatusCode, String)> {
    match app_state.db.create("user").content(register_data).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn setup_surrealdb(args: Args) -> Surreal<Client> {
    let host = args.surrealdb_url.host_str().unwrap();
    let port = args.surrealdb_url.port().unwrap();
    let db_url = format!("{}:{}", host, port);
    let db = Surreal::new::<Ws>(db_url).await.unwrap();
    db.signin(Root {
        username: &args.surrealdb_username,
        password: &args.surrealdb_password,
    })
    .await
    .unwrap();

    db.use_ns(&args.surrealdb_namespace)
        .use_db(&args.surrealdb_database)
        .await
        .unwrap();

    db
}
