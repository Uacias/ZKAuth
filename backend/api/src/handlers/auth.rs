use std::sync::Arc;

use axum::{extract::State, Json};
use reqwest::StatusCode;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::models::user::RegisterUserData;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Surreal<Client>,
}

pub async fn register_no_hashing(
    State(app_state): State<Arc<AppState>>,
    Json(register_data): Json<RegisterUserData>,
) -> Result<(StatusCode, Json<Vec<RegisterUserData>>), (StatusCode, String)> {
    match app_state.db.create("user").content(register_data).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn login_no_hashing(
    State(app_state): State<Arc<AppState>>,
    Json(login_data): Json<RegisterUserData>,
) -> Result<(StatusCode, Json<Vec<RegisterUserData>>), (StatusCode, String)> {
    let sql = "
        SELECT * FROM type::table($table)
        WHERE login = type::String($login) AND password = type::String($password)
    ";

    let mut result = match app_state
        .db
        .query(sql)
        .bind(("table", "user"))
        .bind(("login", &login_data.login))
        .bind(("password", &login_data.password))
        .await
    {
        Ok(result) => result,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    let data: Vec<RegisterUserData> = match result.take(0) {
        Ok(data) => data,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok((StatusCode::OK, Json(data)))
}
