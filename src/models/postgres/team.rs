use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::player::GameType;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub logo: Option<String>,
    pub captain_id: Option<Uuid>,
    pub primary_game: GameType,
    pub region: Option<String>,
    pub status: TeamStatus,
    pub total_earnings: Decimal,
    pub aegis_rating: i32,
    pub bio: Option<String>,
    pub looking_for_players: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "team_status")]
pub enum TeamStatus {
    #[sea_orm(string_value = "active")]
    Active,
    #[sea_orm(string_value = "inactive")]
    Inactive,
    #[sea_orm(string_value = "disbanded")]
    Disbanded,
    #[sea_orm(string_value = "looking_for_players")]
    LookingForPlayers,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::player::Entity")]
    Players,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::CaptainId",
        to = "super::player::Column::Id"
    )]
    Captain,
    #[sea_orm(has_many = "super::tournament_team::Entity")]
    TournamentTeams,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Players.def()
    }
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TournamentTeams.def().rev()
    }
    
    fn via() -> Option<RelationDef> {
        Some(super::tournament_team::Relation::Team.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
