use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::AppState;

pub async fn get_players(
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::http::StatusCode> {
    Ok(Json(json!({
        "players": [],
        "total": 0
    })))
}

pub async fn get_player_by_id(
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::http::StatusCode> {
    Ok(Json(json!({
        "player": null
    })))
}
