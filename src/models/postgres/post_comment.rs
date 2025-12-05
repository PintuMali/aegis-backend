use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub upvotes: i32,
    pub downvotes: i32,
    pub reply_count: i32,
    pub deleted_at: Option<ChronoDateTimeUtc>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::community_post::Entity",
        from = "Column::PostId",
        to = "super::community_post::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::AuthorId",
        to = "super::player::Column::Id"
    )]
    Author,
}

impl Related<super::community_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
