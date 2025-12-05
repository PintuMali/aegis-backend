use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "community_members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub community_id: Uuid,
    pub player_id: Uuid,
    pub role: String,
    pub joined_at: ChronoDateTimeUtc,
    pub banned_until: Option<ChronoDateTimeUtc>,
    pub ban_reason: Option<String>,
    pub permissions: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::community::Entity",
        from = "Column::CommunityId",
        to = "super::community::Column::Id"
    )]
    Community,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::PlayerId",
        to = "super::player::Column::Id"
    )]
    Player,
}

impl Related<super::community::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Community.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Player.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
