pub mod auth;
pub mod chat;
pub mod communities;
pub mod players;
pub mod post;
pub mod tournaments;
pub mod uploads;

pub use chat::*;
pub use communities::*;
pub use post::*;
pub use uploads::*;

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

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
