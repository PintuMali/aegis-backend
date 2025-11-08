use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub mongodb: MongoConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongoConfig {
    pub url: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Settings {
            server: ServerConfig {
                host: env::var("AEGIS_SERVER__HOST")?,
                port: env::var("AEGIS_SERVER__PORT")?.parse()?,
            },
            database: DatabaseConfig {
                url: env::var("AEGIS_DATABASE__URL")?,
                max_connections: env::var("AEGIS_DATABASE__MAX_CONNECTIONS")?.parse()?,
            },
            mongodb: MongoConfig {
                url: env::var("AEGIS_MONGODB__URL")?,
                database: env::var("AEGIS_MONGODB__DATABASE")?,
            },
            jwt: JwtConfig {
                secret: env::var("AEGIS_JWT__SECRET")?,
                expiration: env::var("AEGIS_JWT__EXPIRATION")?.parse()?,
            },
        })
    }
}
