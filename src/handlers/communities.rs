use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use crate::AppState;
use super::chat::ApiResponse;

#[derive(Deserialize)]
pub struct CreateCommunityRequest {
    pub name: String,
    pub description: String,
    pub community_type: String,
    pub owner: String,
}

#[derive(Deserialize)]
pub struct AddPostToCommunityRequest {
    pub post_id: String,
    pub pinned: Option<bool>,
}

pub async fn create_community(
    State(state): State<AppState>,
    Json(payload): Json<CreateCommunityRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.community_service.create_community(
        payload.name,
        payload.description,
        payload.community_type,
        payload.owner,
    ).await {
        Ok(community_id) => Ok(Json(ApiResponse::success(community_id))),
        Err(e) => {
            tracing::error!("Failed to create community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_community(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
) -> Result<Json<ApiResponse<crate::models::dynamodb::Community>>, StatusCode> {
    match state.community_service.get_community(&community_id).await {
        Ok(Some(community)) => Ok(Json(ApiResponse::success(community))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_post_to_community(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
    Json(payload): Json<AddPostToCommunityRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let pinned = payload.pinned.unwrap_or(false);
    
    match state.community_service.add_post_to_community(
        community_id,
        payload.post_id,
        pinned,
    ).await {
        Ok(id) => Ok(Json(ApiResponse::success(id))),
        Err(e) => {
            tracing::error!("Failed to add post to community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_community_posts(
    State(state): State<AppState>,
    Path(community_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<crate::models::dynamodb::CommunityPost>>>, StatusCode> {
    match state.community_service.get_community_posts(&community_id).await {
        Ok(posts) => Ok(Json(ApiResponse::success(posts))),
        Err(e) => {
            tracing::error!("Failed to get community posts: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn join_community(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.community_service.join_community(&community_id, &user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success("Joined community successfully".to_string()))),
        Err(e) => {
            tracing::error!("Failed to join community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn leave_community(
    State(state): State<AppState>,
    Path((community_id, user_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.community_service.leave_community(&community_id, &user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success("Left community successfully".to_string()))),
        Err(e) => {
            tracing::error!("Failed to leave community: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
