pub mod api;

use axum::Router;
use crate::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1", api::routes())
}
