use axum::{
    routing::{get, post},
    Router,
};
use crate::{handlers, AppState};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Chat routes
        .route("/chats", post(handlers::create_chat))
        .route("/chats/:chat_id", get(handlers::get_chat))
        .route("/chats/:chat_id/messages", post(handlers::send_message))
        .route("/chats/:chat_id/messages", get(handlers::get_messages))
        .route("/chats/:chat_id/join/:user_id", post(handlers::join_chat))
        
        // Community routes
        .route("/communities", post(handlers::create_community))
        .route("/communities/:community_id", get(handlers::get_community))
        .route("/communities/:community_id/posts", post(handlers::add_post_to_community))
        .route("/communities/:community_id/posts", get(handlers::get_community_posts))
        .route("/communities/:community_id/join/:user_id", post(handlers::join_community))
        .route("/communities/:community_id/leave/:user_id", post(handlers::leave_community))
        
        // Upload routes
        .route("/uploads/profile/:user_id", post(handlers::upload_profile_picture))
        .route("/uploads/chat/:chat_id", post(handlers::upload_chat_attachment))
        .route("/uploads/presigned/:key", get(handlers::get_presigned_url))
}
