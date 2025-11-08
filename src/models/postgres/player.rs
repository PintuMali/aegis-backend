use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub real_name: Option<String>,
    pub in_game_name: Option<String>,
    pub primary_game: GameType,
    pub aegis_rating: i32,
    pub country: Option<String>,
    pub age: Option<i16>,
    pub team_id: Option<Uuid>,
    pub coins: i64,
    pub verified: bool,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "game_type")]
pub enum GameType {
    #[sea_orm(string_value = "BGMI")]
    Bgmi,
    #[sea_orm(string_value = "VALORANT")]
    Valorant,
    #[sea_orm(string_value = "CS2")]
    Cs2,
    #[sea_orm(string_value = "APEX")]
    Apex,
    #[sea_orm(string_value = "FORTNITE")]
    Fortnite,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id"
    )]
    Team,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
