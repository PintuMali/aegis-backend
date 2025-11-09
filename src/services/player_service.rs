// src/services/player_service.rs
use crate::models::postgres::{player, Player};
use crate::services::auth_service::{AuthService, UserType};
use crate::utils::errors::AppError;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct PlayerService {
    db: DatabaseConnection,
    auth_service: AuthService,
}

impl PlayerService {
    pub fn new(db: DatabaseConnection, auth_service: AuthService) -> Self {
        Self { db, auth_service }
    }

    pub async fn create_player(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<(player::Model, String), AppError> {
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

        let player = new_player.insert(&self.db).await?;

        // Generate JWT with player type
        let token = self
            .auth_service
            .generate_jwt(player.id, UserType::Player, None)?;

        Ok((player, token))
    }

    pub async fn authenticate(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<(player::Model, String)>, AppError> {
        let player = Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        match player {
            Some(p) => {
                if verify(password, &p.password)? {
                    let token = self
                        .auth_service
                        .generate_jwt(p.id, UserType::Player, None)?;
                    Ok(Some((p, token)))
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

    pub async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<player::Model>, AppError> {
        Ok(Player::find()
            .filter(player::Column::Username.eq(username))
            .one(&self.db)
            .await?)
    }

    pub async fn get_all(&self, limit: u64, offset: u64) -> Result<Vec<player::Model>, AppError> {
        Ok(Player::find()
            .order_by_desc(player::Column::AegisRating)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?)
    }

    pub async fn update_profile(
        &self,
        player_id: Uuid,
        update_data: player::ActiveModel,
    ) -> Result<player::Model, AppError> {
        Player::update(update_data)
            .filter(player::Column::Id.eq(player_id))
            .exec(&self.db)
            .await?;

        self.get_by_id(player_id).await?.ok_or(AppError::NotFound)
    }

    pub async fn set_reset_token(
        &self,
        email: String,
        token: String,
        expiry: chrono::DateTime<chrono::Utc>,
    ) -> Result<bool, AppError> {
        // Find the player first
        let player = Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        if let Some(p) = player {
            let mut player_update: player::ActiveModel = p.into();
            player_update.reset_password_token = Set(Some(token));
            player_update.reset_password_expiry = Set(Some(expiry));

            Player::update(player_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn reset_password_with_token(
        &self,
        token: String,
        new_password: String,
    ) -> Result<bool, AppError> {
        let now = chrono::Utc::now();

        // Find player with valid token
        let player = Player::find()
            .filter(player::Column::ResetPasswordToken.eq(Some(token.clone())))
            .filter(player::Column::ResetPasswordExpiry.gt(now))
            .one(&self.db)
            .await?;

        if let Some(p) = player {
            let hashed_password = hash(new_password, DEFAULT_COST)?;

            let mut player_update: player::ActiveModel = p.into();
            player_update.password = Set(hashed_password);
            player_update.reset_password_token = Set(None);
            player_update.reset_password_expiry = Set(None);
            player_update.updated_at = Set(now);

            Player::update(player_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
