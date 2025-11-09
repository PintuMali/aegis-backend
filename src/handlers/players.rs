// src/handlers/players.rs
use crate::services::auth_service::Claims;
use crate::{utils::errors::AppError, AppState};
use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String,
    pub player: PlayerResponse,
}

#[derive(Serialize)]
pub struct PlayerResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let player = state
        .player_service
        .create_player(payload.email, payload.username, payload.password)
        .await?;

    let token = state.auth_service.generate_jwt(player.id)?;

    Ok(Json(AuthResponse {
        message: "Signup successful".to_string(),
        token,
        player: PlayerResponse {
            id: player.id,
            username: player.username,
            email: player.email,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let player = state
        .player_service
        .authenticate(payload.email, payload.password)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let token = state.auth_service.generate_jwt(player.id)?;

    Ok(Json(AuthResponse {
        message: "Login successful".to_string(),
        token,
        player: PlayerResponse {
            id: player.id,
            username: player.username,
            email: player.email,
        },
    }))
}

pub async fn get_player_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PlayerResponse>, AppError> {
    let player = state
        .player_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}

pub async fn get_current_player(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<PlayerResponse>, AppError> {
    let player_id = Uuid::parse_str(&claims.sub)?;
    let player = state
        .player_service
        .get_by_id(player_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}

pub async fn logout() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"message": "Logout successful"})))
}

pub async fn get_player_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<PlayerResponse>, AppError> {
    let player = state
        .player_service
        .get_by_username(username)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerResponse {
        id: player.id,
        username: player.username,
        email: player.email,
    }))
}
