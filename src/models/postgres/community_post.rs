use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "community_posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub community_id: Uuid,
    pub author_id: Uuid,
    pub title: Option<String>,
    pub content: String,
    pub post_type: String,
    pub attachments: Json,
    pub poll_data: Json,
    pub event_data: Json,
    pub tags: Vec<String>,
    pub upvotes: i32,
    pub downvotes: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub pinned: bool,
    pub locked: bool,
    pub deleted_at: Option<ChronoDateTimeUtc>,
    pub metadata: Json,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
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
        from = "Column::AuthorId",
        to = "super::player::Column::Id"
    )]
    Author,
    #[sea_orm(has_many = "super::post_comment::Entity")]
    PostComments,
}

impl Related<super::community::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Community.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
