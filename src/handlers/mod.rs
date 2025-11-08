pub mod auth;
pub mod players;
pub mod tournaments;
pub mod chat;
pub mod post;
pub mod communities;
pub mod uploads;

pub use chat::*;
pub use post::*;
pub use communities::*;
pub use uploads::*;

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use crate::AppState;

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "I'm good man, how are you",
        "service": "aegis-backend",
        "version": "0.1.0",
        "services": {
            "postgresql": "healthy",
            "dynamodb": "healthy", 
            "s3": "healthy"
        }
    })))
}
