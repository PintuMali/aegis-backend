use crate::{handlers, AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Core auth endpoints
        .route("/auth/login", post(handlers::auth_login))
        .route("/auth/register", post(handlers::auth_register))
        .route("/auth/logout", post(handlers::auth_logout))
        .route("/auth/refresh", post(handlers::refresh_token))
        .route("/auth/revoke-sessions", post(handlers::revoke_all_sessions))
        .route("/auth/forgot-password", post(handlers::forgot_password))
        .route(
            "/auth/reset-password/:token",
            post(handlers::reset_password),
        )
        .route("/auth/verify-email/:token", post(handlers::verify_email))
        .route(
            "/auth/send-verification",
            post(handlers::send_verification_email),
        )
        // Player endpoints
        .route("/players/:id", get(handlers::get_player_by_id))
        .route("/players/me", get(handlers::get_current_player))
        .route(
            "/players/username/:username",
            get(handlers::get_player_by_username),
        )
        // Protected resources
        .route("/chats", post(handlers::create_chat))
        .route("/chats/:chat_id", get(handlers::get_chat))
        .route("/chats/:chat_id/messages", post(handlers::send_message))
        .route("/chats/:chat_id/messages", get(handlers::get_messages))
        .route("/chats/:chat_id/join/:user_id", post(handlers::join_chat))
        .route("/communities", post(handlers::create_community))
        .route("/communities/:community_id", get(handlers::get_community))
        .route(
            "/communities/:community_id/posts",
            post(handlers::add_post_to_community),
        )
        .route(
            "/communities/:community_id/posts",
            get(handlers::get_community_posts),
        )
        .route(
            "/communities/:community_id/join/:user_id",
            post(handlers::join_community),
        )
        .route(
            "/communities/:community_id/leave/:user_id",
            post(handlers::leave_community),
        )
        .route(
            "/uploads/profile/:user_id",
            post(handlers::upload_profile_picture),
        )
        .route(
            "/uploads/chat/:chat_id",
            post(handlers::upload_chat_attachment),
        )
        .route("/uploads/presigned/:key", get(handlers::get_presigned_url))
}

pub fn protected_routes() -> Vec<&'static str> {
    vec![
        "/api/v1/players/me",
        "/api/v1/auth/logout",
        "/api/v1/auth/refresh",
        "/api/v1/auth/revoke-sessions",
        "/api/v1/auth/send-verification",
        "/api/v1/chats",
        "/api/v1/chats/:chat_id/messages",
        "/api/v1/chats/:chat_id/join/:user_id",
        "/api/v1/communities",
        "/api/v1/communities/:community_id/posts",
        "/api/v1/communities/:community_id/join/:user_id",
        "/api/v1/communities/:community_id/leave/:user_id",
        "/api/v1/uploads/profile/:user_id",
        "/api/v1/uploads/chat/:chat_id",
    ]
}
