use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use std::env;

mod config;
mod handlers;
mod models;
mod services;
mod middleware;
mod utils;
mod routes;
mod migration;

use config::{Settings, database::{setup_postgres, setup_mongodb}};
use migration::Migrator;
use sea_orm_migration::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,  
    pub mongo_client: mongodb::Client,
    pub settings: Settings,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Check for migration command
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "migrate" && args.len() > 2 && args[2] == "up" {
        return run_migrations().await;
    }

    // Load configuration
    let settings = Settings::new()?;

    // Initialize databases
    let db = setup_postgres(&settings.database.url).await?;
    let mongo_client = setup_mongodb(&settings.mongodb.url).await?;

    // Create application state
    let app_state = AppState {
        db,  // Changed from pg_pool
        mongo_client,
        settings: settings.clone(),
    };


    // Build routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .merge(routes::create_routes())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::cors_layer())
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", settings.server.host, settings.server.port)
    ).await?;

    tracing::info!("🚀 Server running on {}:{}", settings.server.host, settings.server.port);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("AEGIS_DATABASE__URL")
        .expect("AEGIS_DATABASE__URL must be set");
    
    let db = sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&db, None).await?;
    tracing::info!("✅ Migrations completed successfully");
    Ok(())
}


