use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::user::RegisterUserData;

pub async fn register_no_hashing(
    State(app_state): State<Arc<AppState>>,
    Json(register_data): Json<RegisterUserData>,
) -> Result<(StatusCode, Json<Vec<RegisterUserData>>), (StatusCode, String)> {
    match app_state.db.create("user").content(register_data).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Surreal<Client>,
}
