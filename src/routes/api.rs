use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    handlers::{auth, players, tournaments},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        
        // Player routes
        .route("/players", get(players::get_players))
        .route("/players/:id", get(players::get_player_by_id))
        
        // Tournament routes
        .route("/tournaments", get(tournaments::get_tournaments))
}
