// src/models/postgres/player_connection.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_connections")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub requester_id: Uuid,
    pub recipient_id: Uuid,
    pub status: String, // pending, accepted, declined, blocked
    pub message: Option<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::RequesterId",
        to = "super::player::Column::Id"
    )]
    Requester,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::RecipientId",
        to = "super::player::Column::Id"
    )]
    Recipient,
}

impl ActiveModelBehavior for ActiveModel {}
