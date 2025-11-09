// src/services/player_service.rs
use crate::models::postgres::{player, Player};
use crate::utils::errors::AppError;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct PlayerService {
    db: DatabaseConnection,
}

impl PlayerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_player(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<player::Model, AppError> {
        // Check existing email/username
        let existing = Player::find()
            .filter(
                player::Column::Email
                    .eq(&email)
                    .or(player::Column::Username.eq(&username)),
            )
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::Validation(
                "Email or username already exists".to_string(),
            ));
        }

        let hashed_password = hash(password, DEFAULT_COST)?;
        let now = chrono::Utc::now();

        let new_player = player::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(email),
            username: Set(username),
            password: Set(hashed_password),
            verified: Set(false),
            bio: Set(String::new()),
            profile_picture: Set(String::new()),
            earnings: Set(rust_decimal::Decimal::ZERO),
            in_game_role: Set(vec![]),
            languages: Set(vec![]),
            aegis_rating: Set(1000),
            tournaments_played: Set(0),
            battles_played: Set(0),
            qualified_events: Set(false),
            qualified_event_details: Set(vec![]),
            discord_tag: Set(String::new()),
            twitch: Set(String::new()),
            youtube: Set(String::new()),
            twitter: Set(String::new()),
            profile_visibility: Set("public".to_string()),
            card_theme: Set("default".to_string()),
            coins: Set(0),
            check_in_streak: Set(0),
            total_check_ins: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        Ok(new_player.insert(&self.db).await?)
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<player::Model>, AppError> {
        let player = Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        match player {
            Some(p) => {
                if verify(password, &p.password)? {
                    Ok(Some(p))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find_by_id(id).one(&self.db).await?)
    }

    pub async fn get_all(&self, limit: u64, offset: u64) -> Result<Vec<player::Model>, AppError> {
        Ok(Player::find()
            .order_by_desc(player::Column::AegisRating)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?)
    }

    pub async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find()
            .filter(player::Column::Username.eq(username))
            .one(&self.db)
            .await?)
    }
}
