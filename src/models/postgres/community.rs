use crate::models::enums::GameType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "communities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub avatar: String,
    pub banner: String,
    pub game_focus: Option<GameType>,
    pub region: String,
    pub privacy: String,
    pub member_count: i32,
    pub max_members: i32,
    pub owner_id: Uuid,
    pub moderators: Json,
    pub rules: Json,
    pub tags: Vec<String>,
    pub settings: Json,
    pub stats: Json,
    pub social_links: Json,
    pub verified: bool,
    pub featured: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::OwnerId",
        to = "super::player::Column::Id"
    )]
    Owner,
    #[sea_orm(has_many = "super::community_post::Entity")]
    CommunityPosts,
    #[sea_orm(has_many = "super::community_member::Entity")]
    CommunityMembers,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
