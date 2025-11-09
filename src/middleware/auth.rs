// src/middleware/auth.rs
use crate::AppState;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&request)?;
    let claims = state
        .auth_service
        .verify_jwt(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user info to request extensions
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn extract_token(request: &Request) -> Result<String, StatusCode> {
    // Check Authorization header first
    if let Some(auth_header) = request.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Ok(auth_str.replace("Bearer ", ""));
            }
        }
    }

    // TODO: Add cookie support if needed
    Err(StatusCode::UNAUTHORIZED)
}
