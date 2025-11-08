pub mod auth;
pub mod players;
pub mod tournaments;

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "service": "aegis-backend",
        "version": "0.1.0"
    })))
}
