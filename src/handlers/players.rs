use crate::services::auth_service::Claims;
use crate::{utils::errors::AppError, AppState};
use axum::extract::Extension;
use axum::{
    extract::{Path, State},
    http::{header::SET_COOKIE, HeaderMap},
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

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub new_password: String,
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

fn create_auth_cookie(token: &str) -> String {
    format!(
        "token={}; HttpOnly; SameSite=Lax; Max-Age={}; Path=/; Secure",
        token,
        7 * 24 * 60 * 60 // 7 days
    )
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let (player, token) = state
        .player_service
        .create_player(payload.email, payload.username, payload.password)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, create_auth_cookie(&token).parse().unwrap());

    Ok((
        headers,
        Json(AuthResponse {
            message: "Signup successful".to_string(),
            token,
            player: PlayerResponse {
                id: player.id,
                username: player.username,
                email: player.email,
            },
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let (player, token) = state
        .player_service
        .authenticate(payload.email, payload.password)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, create_auth_cookie(&token).parse().unwrap());

    Ok((
        headers,
        Json(AuthResponse {
            message: "Login successful".to_string(),
            token,
            player: PlayerResponse {
                id: player.id,
                username: player.username,
                email: player.email,
            },
        }),
    ))
}

pub async fn logout() -> Result<(HeaderMap, Json<serde_json::Value>), AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        "token=; HttpOnly; SameSite=Lax; Max-Age=0; Path=/; Secure"
            .parse()
            .unwrap(),
    );

    Ok((
        headers,
        Json(serde_json::json!({"message": "Logout successful"})),
    ))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if let Some(reset_token) = state
        .player_service
        .request_password_reset(payload.email.clone())
        .await?
    {
        state
            .email_service
            .send_password_reset(&payload.email, &reset_token)
            .await?;
    }

    Ok(Json(serde_json::json!({
        "message": "If an account with that email exists, a password reset link has been sent."
    })))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let success = state
        .player_service
        .reset_password_with_token(token, payload.new_password)
        .await?;

    if success {
        Ok(Json(
            serde_json::json!({"message": "Password reset successful"}),
        ))
    } else {
        Err(AppError::Validation(
            "Invalid or expired reset token".to_string(),
        ))
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let success = state.player_service.verify_email_by_token(token).await?;

    if success {
        Ok(Json(
            serde_json::json!({"message": "Email verified successfully"}),
        ))
    } else {
        Err(AppError::Validation(
            "Invalid verification token".to_string(),
        ))
    }
}

pub async fn send_verification_email(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, AppError> {
    let player_id = Uuid::parse_str(&claims.sub)?;
    let verification_token = state
        .player_service
        .send_verification_email(player_id)
        .await?;

    let player = state
        .player_service
        .get_by_id(player_id)
        .await?
        .ok_or(AppError::NotFound)?;
    state
        .email_service
        .send_verification_email(&player.email, &verification_token)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Verification email sent successfully"
    })))
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
