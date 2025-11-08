use clap::{Parser, Subcommand};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use mongodb::Client as MongoClient;
use anyhow::Result;

use crate::migration::{Migrator, mongodb_migrator::MongoMigrator};

#[derive(Parser)]
#[command(name = "aegis-migrate")]
#[command(about = "Aegis Backend Migration Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run PostgreSQL migrations
    Postgres {
        #[arg(short, long, default_value = "up")]
        direction: String,
    },
    /// Run MongoDB migrations
    Mongodb,
    /// Run all migrations
    All,
    /// Check migration status
    Status,
    /// Reset all databases (DANGER)
    Reset {
        #[arg(long)]
        confirm: bool,
    },
}

pub async fn run_migrations(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Postgres { direction } => {
            let db = get_postgres_connection().await?;
            
            match direction.as_str() {
                "up" => {
                    Migrator::up(&db, None).await?;
                    println!("✅ PostgreSQL migrations completed");
                }
                "down" => {
                    Migrator::down(&db, None).await?;
                    println!("✅ PostgreSQL rollback completed");
                }
                _ => return Err(anyhow::anyhow!("Invalid direction: {}", direction)),
            }
        }
        Commands::Mongodb => {
            let client = get_mongodb_client().await?;
            let migrator = MongoMigrator::new(client, "aegis_social");
            migrator.migrate().await?;
            println!("✅ MongoDB migrations completed");
        }
        Commands::All => {
            // Run PostgreSQL first
            let db = get_postgres_connection().await?;
            Migrator::up(&db, None).await?;
            println!("✅ PostgreSQL migrations completed");

            // Then MongoDB
            let client = get_mongodb_client().await?;
            let migrator = MongoMigrator::new(client, "aegis_social");
            migrator.migrate().await?;
            println!("✅ MongoDB migrations completed");
            
            println!("🚀 All migrations completed successfully!");
        }
        Commands::Status => {
            check_migration_status().await?;
        }
        Commands::Reset { confirm } => {
            if !confirm {
                return Err(anyhow::anyhow!("Use --confirm flag to reset databases"));
            }
            reset_databases().await?;
        }
    }
    Ok(())
}

async fn get_postgres_connection() -> Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("AEGIS_DATABASE__URL"))
        .expect("DATABASE_URL or AEGIS_DATABASE__URL must be set");
    Ok(Database::connect(&database_url).await?)
}

async fn get_mongodb_client() -> Result<MongoClient> {
    let mongodb_url = std::env::var("MONGODB_URL")
        .or_else(|_| std::env::var("AEGIS_MONGODB__URL"))
        .expect("MONGODB_URL or AEGIS_MONGODB__URL must be set");
    Ok(MongoClient::with_uri_str(&mongodb_url).await?)
}


async fn check_migration_status() -> Result<()> {
    println!("🔍 Checking migration status...");
    
    // Check PostgreSQL
    let db = get_postgres_connection().await?;
    let applied = Migrator::get_applied_migrations(&db).await?;
    let pending = Migrator::get_pending_migrations(&db).await?;
    
    println!("PostgreSQL:");
    println!("  Applied: {}", applied.len());
    println!("  Pending: {}", pending.len());
    
    // Check MongoDB
    let client = get_mongodb_client().await?;
    let db = client.database("aegis_social");
    let collections = db.list_collection_names(None).await?;
    
    println!("MongoDB:");
    println!("  Collections: {}", collections.len());
    println!("  Collections: {:?}", collections);
    
    Ok(())
}

async fn reset_databases() -> Result<()> {
    println!("⚠️  RESETTING ALL DATABASES...");
    
    // Reset PostgreSQL
    let db = get_postgres_connection().await?;
    Migrator::down(&db, None).await?;
    println!("✅ PostgreSQL reset completed");
    
    // Reset MongoDB
    let client = get_mongodb_client().await?;
    let db = client.database("aegis_social");
    db.drop(None).await?;
    println!("✅ MongoDB reset completed");
    
    println!("🔄 Databases reset successfully");
    Ok(())
}
