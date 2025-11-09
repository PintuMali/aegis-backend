use crate::utils::errors::AppError;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,          // user id
    pub user_type: String,    // "player", "admin", "organization"
    pub role: Option<String>, // admin role or organization status
    pub exp: usize,           // expiration
    pub iat: usize,           // issued at
}

#[derive(Debug, Clone)]
pub enum UserType {
    Player,
    Admin,
    Organization,
}

impl UserType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserType::Player => "player",
            UserType::Admin => "admin",
            UserType::Organization => "organization",
        }
    }
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiration: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
        }
    }

    pub fn generate_jwt(
        &self,
        user_id: Uuid,
        user_type: UserType,
        role: Option<String>,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::days(self.jwt_expiration)).timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            user_type: user_type.as_str().to_string(),
            role,
            exp,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalServerError)
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}
