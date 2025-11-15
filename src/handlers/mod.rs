pub mod auth;
pub mod chat;
pub mod communities;
pub mod players;
pub mod post;
pub mod tournaments;
pub mod uploads;

pub use auth::{
    login as auth_login, logout as auth_logout, refresh_token, register as auth_register,
    revoke_all_sessions,
};
pub use chat::*;
pub use communities::*;
pub use players::{
    forgot_password, get_current_player, get_player_by_id, get_player_by_username, reset_password,
    send_verification_email, verify_email,
};
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
