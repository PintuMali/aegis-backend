use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use anyhow::Result;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}

pub async fn login(
    State(_state): State<AppState>,
    Json(_payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, axum::http::StatusCode> {
    // TODO: Implement authentication logic
    Ok(Json(LoginResponse {
        token: "dummy_token".to_string(),
        user_id: "dummy_id".to_string(),
    }))
}

pub async fn register(
    State(_state): State<AppState>,
    Json(_payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, axum::http::StatusCode> {
    // TODO: Implement registration logic
    Ok(Json(LoginResponse {
        token: "dummy_token".to_string(),
        user_id: "dummy_id".to_string(),
    }))
}
