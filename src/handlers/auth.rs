use crate::services::auth_service::{Claims, UserType};
use crate::{utils::errors::AppError, AppState};
use axum::{
    extract::{ConnectInfo, Extension, State},
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub user_type: String, // "player", "organization"
    // Player fields
    pub username: Option<String>,
    // Organization fields
    pub org_name: Option<String>,
    pub owner_name: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: String,
    pub refresh_token: String,
    pub session_id: String,
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub refresh_token: String,
    pub session_id: String,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub org_name: Option<String>,
    pub user_type: String,
    pub verified: bool,
    pub approval_status: Option<String>,
}

fn create_auth_cookie(token: &str) -> String {
    format!(
        "token={}; HttpOnly; SameSite=Lax; Max-Age={}; Path=/; Secure",
        token,
        7 * 24 * 60 * 60 // 7 days
    )
}

fn extract_client_info(
    headers: &HeaderMap,
    addr: Option<SocketAddr>,
) -> (Option<String>, Option<String>) {
    let ip_address = addr.map(|a| a.ip().to_string());
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    (ip_address, user_agent)
}

fn create_auth_response_with_session(
    token: String,
    session: crate::models::postgres::user_session::Model,
    user: UserInfo,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        create_auth_cookie(&token).parse().unwrap(),
    );

    let message = match user.user_type.as_str() {
        "player" => "Player registration successful. Please verify your email.",
        "organization" => "Organization registration successful. Pending admin approval.",
        "admin" => "Admin login successful.",
        _ => "Authentication successful.",
    };

    Ok((
        headers,
        Json(AuthResponse {
            message: message.to_string(),
            token,
            refresh_token: session.refresh_token,
            session_id: session.id.to_string(),
            user,
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let (ip_address, user_agent) = extract_client_info(&headers, Some(addr));

    // Rate limiting check
    if let Some(ip) = &ip_address {
        state
            .rate_limit_service
            .check_rate_limit(
                ip.clone(),
                "ip".to_string(),
                "login".to_string(),
                5,  // 5 attempts per hour
                60, // 60 minutes window
            )
            .await?;
    }

    // Try player authentication first
    if let Some((player, _)) = state
        .player_service
        .authenticate(payload.email.clone(), payload.password.clone())
        .await?
    {
        let session = state
            .session_service
            .create_session(
                player.id,
                "player".to_string(),
                ip_address.clone(),
                user_agent.clone(),
            )
            .await?;

        // ✅ FIXED: Generate JWT with actual session ID
        let token = state.auth_service.generate_jwt(
            player.id,
            UserType::Player,
            None,
            session.id.to_string(),
        )?;

        // Audit log
        let _ = state
            .audit_service
            .log_action(
                Some(player.id),
                Some("player".to_string()),
                Some(session.id),
                "login".to_string(),
                Some("player".to_string()),
                Some(player.id),
                ip_address.clone(),
                user_agent.clone(),
                true,
                None,
                None,
                None,
            )
            .await;

        return create_auth_response_with_session(
            token,
            session,
            UserInfo {
                id: player.id,
                email: player.email,
                username: Some(player.username),
                org_name: None,
                user_type: "player".to_string(),
                verified: player.verified,
                approval_status: None,
            },
        );
    }

    // Try admin authentication
    if let Some((admin, _)) = state
        .admin_service
        .authenticate(payload.email.clone(), payload.password.clone())
        .await?
    {
        let session = state
            .session_service
            .create_session(
                admin.id,
                "admin".to_string(),
                ip_address.clone(),
                user_agent.clone(),
            )
            .await?;

        // ✅ FIXED: Generate JWT with actual session ID
        let token = state.auth_service.generate_jwt(
            admin.id,
            UserType::Admin,
            Some(admin.role.clone()),
            session.id.to_string(),
        )?;

        // Audit log
        let _ = state
            .audit_service
            .log_action(
                Some(admin.id),
                Some("admin".to_string()),
                Some(session.id),
                "login".to_string(),
                Some("admin".to_string()),
                Some(admin.id),
                ip_address.clone(),
                user_agent.clone(),
                true,
                None,
                None,
                None,
            )
            .await;

        return create_auth_response_with_session(
            token,
            session,
            UserInfo {
                id: admin.id,
                email: admin.email,
                username: Some(admin.username),
                org_name: None,
                user_type: "admin".to_string(),
                verified: true,
                approval_status: Some(if admin.is_active {
                    "active".to_string()
                } else {
                    "inactive".to_string()
                }),
            },
        );
    }

    // Try organization authentication
    if let Some((org, _)) = state
        .organization_service
        .authenticate(payload.email.clone(), payload.password)
        .await?
    {
        let session = state
            .session_service
            .create_session(
                org.id,
                "organization".to_string(),
                ip_address.clone(),
                user_agent.clone(),
            )
            .await?;

        // ✅ FIXED: Generate JWT with actual session ID
        let token = state.auth_service.generate_jwt(
            org.id,
            UserType::Organization,
            None,
            session.id.to_string(),
        )?;

        // Audit log
        let _ = state
            .audit_service
            .log_action(
                Some(org.id),
                Some("organization".to_string()),
                Some(session.id),
                "login".to_string(),
                Some("organization".to_string()),
                Some(org.id),
                ip_address.clone(),
                user_agent.clone(),
                true,
                None,
                None,
                None,
            )
            .await;

        return create_auth_response_with_session(
            token,
            session,
            UserInfo {
                id: org.id,
                email: org.email,
                username: None,
                org_name: Some(org.org_name),
                user_type: "organization".to_string(),
                verified: org.email_verified,
                approval_status: Some(org.approval_status),
            },
        );
    }

    // Failed login audit
    let _ = state
        .audit_service
        .log_action(
            None,
            None,
            None,
            "login".to_string(),
            None,
            None,
            ip_address,
            user_agent,
            false,
            Some("Invalid credentials".to_string()),
            None,
            None,
        )
        .await;

    Err(AppError::Unauthorized)
}

pub async fn register(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let (ip_address, user_agent) = extract_client_info(&headers, Some(addr));

    // Rate limiting check
    if let Some(ip) = &ip_address {
        state
            .rate_limit_service
            .check_rate_limit(
                ip.clone(),
                "ip".to_string(),
                "register".to_string(),
                3,  // 3 registrations per hour
                60, // 60 minutes window
            )
            .await?;
    }

    match payload.user_type.as_str() {
        "player" => register_player(state, payload, ip_address, user_agent).await,
        "organization" => register_organization(state, payload, ip_address, user_agent).await,
        _ => Err(AppError::Validation("Invalid user type".to_string())),
    }
}

async fn register_player(
    state: AppState,
    payload: RegisterRequest,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    println!("DEBUG: Starting register_player handler");

    let username = payload.username.ok_or_else(|| {
        AppError::Validation("Username required for player registration".to_string())
    })?;

    println!("DEBUG: About to call player_service.create_player");
    let (player, _) = state
        .player_service
        .create_player(payload.email, username, payload.password)
        .await?;
    println!("DEBUG: Player created successfully, ID: {}", player.id);

    println!("DEBUG: About to create session");
    let session = match state
        .session_service
        .create_session(
            player.id,
            "player".to_string(),
            ip_address.clone(),
            user_agent.clone(),
        )
        .await
    {
        Ok(s) => {
            println!("DEBUG: Session created successfully, ID: {}", s.id);
            s
        }
        Err(e) => {
            println!("DEBUG: Session creation failed: {:?}", e);
            return Err(e);
        }
    };

    println!("DEBUG: About to generate JWT token");
    let token = match state.auth_service.generate_jwt(
        player.id,
        UserType::Player,
        None,
        session.id.to_string(),
    ) {
        Ok(t) => {
            println!("DEBUG: JWT generated successfully (length: {})", t.len());
            t
        }
        Err(e) => {
            println!("DEBUG: JWT generation failed: {:?}", e);
            return Err(e);
        }
    };

    println!("DEBUG: About to send verification email");
    let verification_token = match state
        .player_service
        .send_verification_email(player.id)
        .await
    {
        Ok(token) => {
            println!("DEBUG: Verification token generated successfully");
            token
        }
        Err(e) => {
            println!("DEBUG: Verification token generation failed: {:?}", e);
            return Err(e);
        }
    };

    println!("DEBUG: About to send email");
    let _ = state
        .email_service
        .send_verification_email(&player.email, &verification_token)
        .await;
    println!("DEBUG: Email sending completed (may have failed silently)");

    println!("DEBUG: About to log audit action");
    let _ = state
        .audit_service
        .log_action(
            Some(player.id),
            Some("player".to_string()),
            Some(session.id),
            "register".to_string(),
            Some("player".to_string()),
            Some(player.id),
            ip_address,
            user_agent,
            true,
            None,
            None,
            None,
        )
        .await;
    println!("DEBUG: Audit logging completed");

    println!("DEBUG: About to create auth response");
    let result = create_auth_response_with_session(
        token,
        session,
        UserInfo {
            id: player.id,
            email: player.email,
            username: Some(player.username),
            org_name: None,
            user_type: "player".to_string(),
            verified: player.verified,
            approval_status: None,
        },
    );

    match &result {
        Ok(_) => println!("DEBUG: Auth response created successfully"),
        Err(e) => println!("DEBUG: Auth response creation failed: {:?}", e),
    }

    result
}

async fn register_organization(
    state: AppState,
    payload: RegisterRequest,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(HeaderMap, Json<AuthResponse>), AppError> {
    let org_name = payload
        .org_name
        .ok_or_else(|| AppError::Validation("Organization name required".to_string()))?;
    let owner_name = payload
        .owner_name
        .ok_or_else(|| AppError::Validation("Owner name required".to_string()))?;
    let country = payload
        .country
        .ok_or_else(|| AppError::Validation("Country required".to_string()))?;
    let description = payload
        .description
        .ok_or_else(|| AppError::Validation("Description required".to_string()))?;

    let (org, _) = state
        .organization_service
        .create_organization(
            org_name,
            owner_name,
            payload.email,
            payload.password,
            country,
            description,
        )
        .await?;

    let session = state
        .session_service
        .create_session(
            org.id,
            "organization".to_string(),
            ip_address.clone(),
            user_agent.clone(),
        )
        .await?;

    // ✅ FIXED: Generate JWT with actual session ID
    let token = state.auth_service.generate_jwt(
        org.id,
        UserType::Organization,
        None,
        session.id.to_string(),
    )?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            Some(org.id),
            Some("organization".to_string()),
            Some(session.id),
            "register".to_string(),
            Some("organization".to_string()),
            Some(org.id),
            ip_address,
            user_agent,
            true,
            None,
            None,
            None,
        )
        .await;

    create_auth_response_with_session(
        token,
        session,
        UserInfo {
            id: org.id,
            email: org.email,
            username: None,
            org_name: Some(org.org_name),
            user_type: "organization".to_string(),
            verified: org.email_verified,
            approval_status: Some(org.approval_status),
        },
    )
}

pub async fn refresh_token(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<(HeaderMap, Json<TokenResponse>), AppError> {
    let (ip_address, user_agent) = extract_client_info(&headers, Some(addr));

    let session = state
        .session_service
        .refresh_session(&payload.refresh_token)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let user_id = session.user_id;
    let user_type = &session.user_type;

    let new_token = match user_type.as_str() {
        "player" => {
            let player = state
                .player_service
                .get_by_id(user_id)
                .await?
                .ok_or(AppError::Unauthorized)?;
            state.auth_service.generate_jwt(
                player.id,
                UserType::Player,
                None,
                session.id.to_string(),
            )?
        }
        "admin" => {
            let admin = state
                .admin_service
                .get_by_id(user_id)
                .await?
                .ok_or(AppError::Unauthorized)?;
            state.auth_service.generate_jwt(
                admin.id,
                UserType::Admin,
                Some(admin.role),
                session.id.to_string(),
            )?
        }
        "organization" => {
            let org = state
                .organization_service
                .get_by_id(user_id)
                .await?
                .ok_or(AppError::Unauthorized)?;
            state.auth_service.generate_jwt(
                org.id,
                UserType::Organization,
                None,
                session.id.to_string(),
            )?
        }
        _ => return Err(AppError::Unauthorized),
    };

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            Some(user_id),
            Some(user_type.clone()),
            Some(session.id),
            "refresh_token".to_string(),
            Some("session".to_string()),
            Some(session.id),
            ip_address,
            user_agent,
            true,
            None,
            None,
            None,
        )
        .await;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        create_auth_cookie(&new_token).parse().unwrap(),
    );

    Ok((
        headers,
        Json(TokenResponse {
            token: new_token,
            refresh_token: session.refresh_token,
            session_id: session.id.to_string(),
        }),
    ))
}

pub async fn revoke_all_sessions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    state
        .session_service
        .revoke_all_user_sessions(user_id)
        .await?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            Some(user_id),
            Some(claims.user_type.clone()),
            Some(Uuid::parse_str(&claims.session_id)?),
            "revoke_all_sessions".to_string(),
            Some("session".to_string()),
            Some(user_id),
            None,
            None,
            true,
            None,
            None,
            None,
        )
        .await;

    Ok(Json(serde_json::json!({
        "message": "All sessions revoked successfully"
    })))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<(HeaderMap, Json<serde_json::Value>), AppError> {
    let session_id = Uuid::parse_str(&claims.session_id)?;
    let user_id = Uuid::parse_str(&claims.sub)?;

    // Revoke current session
    state
        .session_service
        .revoke_session(&claims.session_id)
        .await?;

    // Audit log
    let _ = state
        .audit_service
        .log_action(
            Some(user_id),
            Some(claims.user_type.clone()),
            Some(session_id),
            "logout".to_string(),
            Some("session".to_string()),
            Some(session_id),
            None,
            None,
            true,
            None,
            None,
            None,
        )
        .await;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        "token=; HttpOnly; SameSite=Lax; Max-Age=0; Path=/; Secure"
            .parse()
            .unwrap(),
    );

    Ok((
        headers,
        Json(serde_json::json!({
            "message": "Logout successful"
        })),
    ))
}
