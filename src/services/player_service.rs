use crate::models::postgres::{player, Player};
use crate::services::auth_service::{AuthService, UserType};
use crate::utils::errors::AppError;
use crate::utils::validation::{validate_email, validate_password, validate_username};
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
        println!(
            "DEBUG: Starting create_player with email: {}, username: {}",
            email, username
        );

        // Validate inputs
        println!("DEBUG: Validating email: {}", email);
        match validate_email(&email) {
            Ok(_) => println!("DEBUG: Email validation passed"),
            Err(e) => {
                println!("DEBUG: Email validation failed: {:?}", e);
                return Err(e);
            }
        }

        println!("DEBUG: Validating username: {}", username);
        match validate_username(&username) {
            Ok(_) => println!("DEBUG: Username validation passed"),
            Err(e) => {
                println!("DEBUG: Username validation failed: {:?}", e);
                return Err(e);
            }
        }

        println!("DEBUG: Validating password (length: {})", password.len());
        match validate_password(&password) {
            Ok(_) => println!("DEBUG: Password validation passed"),
            Err(e) => {
                println!("DEBUG: Password validation failed: {:?}", e);
                return Err(e);
            }
        }

        // Check existing email/username
        println!("DEBUG: Checking for existing user with email or username");
        let existing = match Player::find()
            .filter(
                player::Column::Email
                    .eq(&email)
                    .or(player::Column::Username.eq(&username)),
            )
            .one(&self.db)
            .await
        {
            Ok(result) => {
                println!("DEBUG: Database query for existing user completed successfully");
                result
            }
            Err(e) => {
                println!("DEBUG: Database query for existing user failed: {:?}", e);
                return Err(AppError::Database(e));
            }
        };

        if existing.is_some() {
            println!("DEBUG: User already exists with this email or username");
            return Err(AppError::Validation(
                "Email or username already exists".to_string(),
            ));
        }
        println!("DEBUG: No existing user found, proceeding with creation");

        println!("DEBUG: Hashing password");
        let hashed_password = match self.auth_service.hash_password(&password) {
            Ok(hash) => {
                println!(
                    "DEBUG: Password hashed successfully (length: {})",
                    hash.len()
                );
                hash
            }
            Err(e) => {
                println!("DEBUG: Password hashing failed: {:?}", e);
                return Err(e);
            }
        };

        let now = chrono::Utc::now();
        println!("DEBUG: Current timestamp: {}", now);

        // Create the ActiveModel
        println!("DEBUG: Creating ActiveModel with all required fields");
        let new_player = player::ActiveModel {
            id: Set(Uuid::new_v4()),
            cognito_sub: Set(None),
            email: Set(email.clone()),
            username: Set(username.clone()),
            in_game_name: Set(None),
            real_name: Set(None),
            password: Set(hashed_password),
            reset_password_token: Set(None),
            reset_password_expiry: Set(None),
            verified: Set(false),
            country: Set(None),
            bio: Set(String::new()),
            profile_picture: Set(String::new()),
            primary_game: NotSet,
            earnings: Set(rust_decimal::Decimal::ZERO),
            in_game_role: Set(vec![]),
            location: Set(None),
            age: Set(None),
            languages: Set(vec![]),
            aegis_rating: Set(1000),
            tournaments_played: Set(0),
            battles_played: Set(0),
            qualified_events: Set(false),
            qualified_event_details: Set(vec![]),
            team_status: Set(None),
            team_id: Set(None),
            availability: Set(None),
            discord_tag: Set(String::new()),
            twitch: Set(String::new()),
            youtube: Set(String::new()),
            twitter: Set(String::new()),
            profile_visibility: Set("public".to_string()),
            card_theme: Set("default".to_string()),
            coins: Set(0),
            last_check_in: Set(None),
            check_in_streak: Set(0),
            total_check_ins: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
        };
        println!("DEBUG: ActiveModel created successfully");

        // Attempt database insert with detailed error handling
        println!("DEBUG: Attempting database insert");
        let player = match new_player.insert(&self.db).await {
            Ok(p) => {
                println!("DEBUG: Database insert successful! Player ID: {}", p.id);
                p
            }
            Err(e) => {
                println!("DEBUG: Database insert failed with detailed error:");
                println!("DEBUG: Error type: {:?}", e);
                println!("DEBUG: Error message: {}", e);

                // Check for specific database errors
                match &e {
                    DbErr::Exec(runtime_err) => {
                        println!("DEBUG: Execution error: {:?}", runtime_err);
                    }
                    DbErr::Query(runtime_err) => {
                        println!("DEBUG: Query error: {:?}", runtime_err);
                    }
                    DbErr::Conn(runtime_err) => {
                        println!("DEBUG: Connection error: {:?}", runtime_err);
                    }
                    DbErr::Type(type_err) => {
                        println!("DEBUG: Type conversion error: {}", type_err);
                    }
                    _ => {
                        println!("DEBUG: Other database error: {:?}", e);
                    }
                }

                return Err(AppError::Database(e));
            }
        };

        // Generate JWT token
        println!("DEBUG: Generating JWT token for player ID: {}", player.id);
        let token = match self.auth_service.generate_jwt(
            player.id,
            UserType::Player,
            None,
            Uuid::new_v4().to_string(),
        ) {
            Ok(t) => {
                println!(
                    "DEBUG: JWT token generated successfully (length: {})",
                    t.len()
                );
                t
            }
            Err(e) => {
                println!("DEBUG: JWT token generation failed: {:?}", e);
                return Err(e);
            }
        };

        println!("DEBUG: create_player completed successfully");
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
                if self.auth_service.verify_password(&password, &p.password)? {
                    let token = self.auth_service.generate_jwt(
                        p.id,
                        UserType::Player,
                        None,
                        Uuid::new_v4().to_string(),
                    )?;
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

    pub async fn send_verification_email(&self, player_id: Uuid) -> Result<String, AppError> {
        let player = self.get_by_id(player_id).await?.ok_or(AppError::NotFound)?;

        if player.verified {
            return Err(AppError::Validation("Email already verified".to_string()));
        }

        let verification_token = uuid::Uuid::new_v4().to_string();
        let expiry = chrono::Utc::now() + chrono::Duration::hours(24);

        let mut player_update: player::ActiveModel = player.into();
        player_update.reset_password_token = Set(Some(format!("verify_{}", verification_token)));
        player_update.reset_password_expiry = Set(Some(expiry));

        Player::update(player_update).exec(&self.db).await?;
        Ok(verification_token)
    }

    pub async fn verify_email_by_token(&self, token: String) -> Result<bool, AppError> {
        let now = chrono::Utc::now();
        let verification_token = format!("verify_{}", token);

        let player = Player::find()
            .filter(player::Column::ResetPasswordToken.eq(Some(verification_token)))
            .filter(player::Column::ResetPasswordExpiry.gt(now))
            .one(&self.db)
            .await?;

        if let Some(p) = player {
            let mut player_update: player::ActiveModel = p.into();
            player_update.verified = Set(true);
            player_update.reset_password_token = Set(None);
            player_update.reset_password_expiry = Set(None);
            player_update.updated_at = Set(now);

            Player::update(player_update).exec(&self.db).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn request_password_reset(&self, email: String) -> Result<Option<String>, AppError> {
        let player = Player::find()
            .filter(player::Column::Email.eq(email))
            .one(&self.db)
            .await?;

        if let Some(p) = player {
            let reset_token = uuid::Uuid::new_v4().to_string();
            let expiry = chrono::Utc::now() + chrono::Duration::hours(1);

            let mut player_update: player::ActiveModel = p.into();
            player_update.reset_password_token = Set(Some(reset_token.clone()));
            player_update.reset_password_expiry = Set(Some(expiry));

            Player::update(player_update).exec(&self.db).await?;
            Ok(Some(reset_token))
        } else {
            Ok(None)
        }
    }

    pub async fn reset_password_with_token(
        &self,
        token: String,
        new_password: String,
    ) -> Result<bool, AppError> {
        validate_password(&new_password)?;

        let now = chrono::Utc::now();
        let player = Player::find()
            .filter(player::Column::ResetPasswordToken.eq(Some(token)))
            .filter(player::Column::ResetPasswordExpiry.gt(now))
            .one(&self.db)
            .await?;

        if let Some(p) = player {
            let hashed_password = self.auth_service.hash_password(&new_password)?;

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
