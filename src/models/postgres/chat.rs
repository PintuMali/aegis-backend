use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub chat_type: String,
    pub name: String,
    pub description: String,
    pub participants: Json,
    pub created_by: Uuid,
    pub team_id: Option<Uuid>,
    pub tournament_id: Option<Uuid>,
    pub community_id: Option<Uuid>,
    pub is_private: bool,
    pub max_participants: i32,
    pub settings: Json,
    pub metadata: Json,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::CreatedBy",
        to = "super::player::Column::Id"
    )]
    CreatedBy,
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id"
    )]
    Team,
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id"
    )]
    Tournament,
    #[sea_orm(has_many = "super::chat_message::Entity")]
    ChatMessages,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CreatedBy.def()
    }
}

impl Related<super::chat_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatMessages.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
