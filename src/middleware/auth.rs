use crate::services::auth_service::Claims;
use crate::utils::errors::AppError;
use crate::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_request(&request)?;

    // Verify JWT
    let claims = state.auth_service.verify_jwt(&token)?;

    // Validate session exists and is active
    let session = state
        .session_service
        .validate_session(&claims.session_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Ensure session belongs to the user
    if session.user_id.to_string() != claims.sub {
        return Err(AppError::Unauthorized);
    }

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

fn extract_token_from_request(request: &Request) -> Result<String, AppError> {
    // Try Authorization header first
    if let Some(auth_header) = request.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Ok(token.to_string());
            }
        }
    }

    // Try cookie as fallback
    if let Some(cookie_header) = request.headers().get("cookie") {
        if let Ok(cookies) = cookie_header.to_str() {
            for cookie in cookies.split(';') {
                let cookie = cookie.trim();
                if let Some(token) = cookie.strip_prefix("token=") {
                    return Ok(token.to_string());
                }
            }
        }
    }

    Err(AppError::Unauthorized)
}

// Admin-only middleware
pub async fn admin_only_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claims.user_type != "admin" {
        return Err(AppError::Unauthorized);
    }

    // Verify admin is still active
    let admin_id = claims
        .sub
        .parse()
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    let admin = state
        .admin_service
        .get_by_id(admin_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !admin.is_active {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}

// Organization-only middleware
pub async fn organization_only_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claims.user_type != "organization" {
        return Err(AppError::Unauthorized);
    }

    // Verify organization is approved
    let org_id = claims
        .sub
        .parse()
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    let org = state
        .organization_service
        .get_by_id(org_id)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if org.approval_status != "approved" {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}
