use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub name: String,
    pub chat_type: String,
    pub created_by: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender: String,
    pub message: String,
    pub message_type: Option<String>,
}

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<i32>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
        }
    }
}

pub async fn create_chat(
    State(state): State<AppState>,
    Json(payload): Json<CreateChatRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.chat_service.create_chat(
        payload.name,
        payload.chat_type,
        payload.created_by,
    ).await {
        Ok(chat_id) => Ok(Json(ApiResponse::success(chat_id))),
        Err(e) => {
            tracing::error!("Failed to create chat: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_chat(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::dynamodb::Chat>>, StatusCode> {
    match state.chat_service.get_chat(&chat_id).await {
        Ok(Some(chat)) => Ok(Json(ApiResponse::success(chat))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get chat: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn send_message(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let message_type = payload.message_type.unwrap_or_else(|| "text".to_string());
    
    match state.chat_service.send_message(
        chat_id,
        payload.sender,
        payload.message,
        message_type,
    ).await {
        Ok(message_id) => Ok(Json(ApiResponse::success(message_id))),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    Query(params): Query<GetMessagesQuery>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::ChatMessage>>>, StatusCode> {
    match state.chat_service.get_messages(&chat_id, params.limit).await {
        Ok(messages) => Ok(Json(ApiResponse::success(messages))),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn join_chat(
    State(state): State<AppState>,
    Path((chat_id, user_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.chat_service.join_chat(&chat_id, &user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success("Joined chat successfully".to_string()))),
        Err(e) => {
            tracing::error!("Failed to join chat: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
