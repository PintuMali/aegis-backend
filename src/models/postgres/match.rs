use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::player::GameType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "matches")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub match_number: i32,
    pub game_type: GameType,
    pub status: MatchStatus,
    pub map_name: Option<String>,
    pub scheduled_start: ChronoDateTimeUtc,
    pub actual_start: Option<ChronoDateTimeUtc>,
    pub actual_end: Option<ChronoDateTimeUtc>,
    pub settings: Option<Json>,
    pub created_at: ChronoDateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "match_status")]
pub enum MatchStatus {
    #[sea_orm(string_value = "scheduled")]
    Scheduled,
    #[sea_orm(string_value = "in_progress")]
    InProgress,
    #[sea_orm(string_value = "completed")]
    Completed,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id"
    )]
    Tournament,
    #[sea_orm(has_many = "super::match_participant::Entity")]
    MatchParticipants,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MatchParticipants.def().rev()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::match_participant::Relation::Match.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
