use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::player::GameType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "player_game_stats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub player_id: Uuid,
    pub game_type: GameType,
    pub rank_tier: Option<String>,
    pub matches_played: i32,
    pub wins: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub damage_dealt: i64,
    pub stats: Option<Json>, // Game-specific stats (K/D, ADR, etc.)
    pub season: Option<String>,
    pub last_updated: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::PlayerId",
        to = "super::player::Column::Id"
    )]
    Player,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
