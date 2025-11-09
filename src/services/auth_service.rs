// src/services/auth_service.rs
use crate::utils::errors::AppError;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiration
    pub iat: usize,  // issued at
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

    pub fn generate_jwt(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::days(self.jwt_expiration)).timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
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
