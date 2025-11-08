use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Json,
};
use crate::AppState;
use super::chat::ApiResponse;

pub async fn upload_profile_picture(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("avatar.jpg").to_string();
            let data = field.bytes().await.unwrap().to_vec();
            
            let extension = filename.split('.').last().unwrap_or("jpg");
            
            match state.s3_service.upload_profile_picture(&user_id, data, extension).await {
                Ok(url) => return Ok(Json(ApiResponse::success(url))),
                Err(e) => {
                    tracing::error!("Failed to upload profile picture: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}

pub async fn upload_chat_attachment(
    State(state): State<AppState>,
    Path(chat_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("attachment").to_string();
            let data = field.bytes().await.unwrap().to_vec();
            
            match state.s3_service.upload_chat_attachment(&chat_id, &filename, data).await {
                Ok(url) => return Ok(Json(ApiResponse::success(url))),
                Err(e) => {
                    tracing::error!("Failed to upload chat attachment: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}

pub async fn get_presigned_url(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.s3_service.get_presigned_url(&key, 3600).await { // 1 hour expiry
        Ok(url) => Ok(Json(ApiResponse::success(url))),
        Err(e) => {
            tracing::error!("Failed to generate presigned URL: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
