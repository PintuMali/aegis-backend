use crate::services::auth_service::Claims;
use crate::utils::errors::AppError;
use crate::AppState; // Add this import
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn get_dashboard_data(
    State(state): State<AppState>, // Change from State<DashboardService>
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)?;

    tracing::info!("📊 Dashboard data request for user: {}", user_id);

    let dashboard_data = state.dashboard_service.get_dashboard_data(user_id).await?;

    Ok(Json(json!({
        "success": true,
        "data": dashboard_data,
    })))
}

pub async fn dashboard_health(
    State(_state): State<AppState>, // Change from State<DashboardService>
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "service": "dashboard",
        "timestamp": chrono::Utc::now()
    })))
}
