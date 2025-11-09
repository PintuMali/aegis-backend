use crate::{models::user_context::UserContext, AppState};
use axum::{
    extract::{Request, State},
    http::{
        header::{AUTHORIZATION, COOKIE},
        StatusCode,
    },
    middleware::Next,
    response::Response,
};
use std::future::Future;
use std::pin::Pin;

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

    let user_id = claims.sub.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    let user_context = match claims.user_type.as_str() {
        "player" => {
            let player = state
                .player_service
                .get_by_id(user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;
            UserContext::Player(player)
        }
        "admin" => {
            let admin = state
                .admin_service
                .get_by_id(user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;

            if !admin.is_active {
                return Err(StatusCode::UNAUTHORIZED);
            }

            if let Some(lock_until) = admin.lock_until {
                if lock_until > chrono::Utc::now() {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }

            UserContext::Admin(admin)
        }
        "organization" => {
            let org = state
                .organization_service
                .get_by_id(user_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::UNAUTHORIZED)?;

            if org.approval_status != "approved" {
                return Err(StatusCode::FORBIDDEN);
            }

            UserContext::Organization(org)
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(user_context);
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn extract_token(request: &Request) -> Result<String, StatusCode> {
    // Check cookies first (Express compatibility)
    if let Some(cookie_header) = request.headers().get(COOKIE) {
        if let Ok(cookies) = cookie_header.to_str() {
            for cookie in cookies.split(';') {
                let parts: Vec<&str> = cookie.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == "token" {
                    return Ok(parts[1].to_string());
                }
            }
        }
    }

    // Then check Authorization header
    if let Some(auth_header) = request.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Ok(auth_str.replace("Bearer ", ""));
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Role-specific middleware for admin-only routes
pub async fn admin_only_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let user_context = request
        .extensions()
        .get::<UserContext>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !user_context.is_admin() {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

// Permission-based middleware for admin routes
pub fn admin_permission_middleware(
    permission: &'static str,
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = Result<Response, StatusCode>> + Send>> + Clone
{
    let permission = permission.to_string();
    move |request: Request, next: Next| {
        let permission = permission.clone();
        Box::pin(async move {
            let user_context = request
                .extensions()
                .get::<UserContext>()
                .ok_or(StatusCode::UNAUTHORIZED)?;

            match user_context {
                UserContext::Admin(admin) => {
                    if let Some(perm_value) = admin.permissions.get(&permission) {
                        if perm_value.as_bool().unwrap_or(false) {
                            Ok(next.run(request).await)
                        } else {
                            Err(StatusCode::FORBIDDEN)
                        }
                    } else {
                        Err(StatusCode::FORBIDDEN)
                    }
                }
                _ => Err(StatusCode::FORBIDDEN),
            }
        })
    }
}
