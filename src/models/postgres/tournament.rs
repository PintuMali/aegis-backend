use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::player::GameType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tournaments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub short_name: Option<String>,
    pub game_type: GameType,
    pub status: TournamentStatus,
    pub organizer_id: Option<Uuid>,
    pub prize_pool_amount: Decimal,
    pub prize_pool_currency: String,
    pub max_teams: i32,
    pub start_date: ChronoDateTimeUtc,
    pub end_date: ChronoDateTimeUtc,
    pub registration_start: Option<ChronoDateTimeUtc>,
    pub registration_end: Option<ChronoDateTimeUtc>,
    pub rules: Option<Json>,
    pub settings: Option<Json>,
    pub featured: bool,
    pub verified: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tournament_status")]
pub enum TournamentStatus {
    #[sea_orm(string_value = "announced")]
    Announced,
    #[sea_orm(string_value = "registration_open")]
    RegistrationOpen,
    #[sea_orm(string_value = "registration_closed")]
    RegistrationClosed,
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
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizerId",
        to = "super::organization::Column::Id"
    )]
    Organizer,
    #[sea_orm(has_many = "super::tournament_team::Entity")]
    TournamentTeams,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizer.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeams.def().rev()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::tournament_team::Relation::Tournament.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
