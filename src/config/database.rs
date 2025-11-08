use sea_orm::{Database, DatabaseConnection};
use mongodb::{Client, options::ClientOptions};
use anyhow::Result;

pub async fn setup_postgres(database_url: &str) -> Result<DatabaseConnection> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub async fn setup_mongodb(mongodb_url: &str) -> Result<Client> {
    let client_options = ClientOptions::parse(mongodb_url).await?;
    let client = Client::with_options(client_options)?;
    
    client.database("admin").run_command(mongodb::bson::doc! {"ping": 1}, None).await?;
    Ok(client)
}
