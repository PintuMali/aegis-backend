use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub message: String,
    pub message_type: String,
    pub reply_to: Option<Uuid>,
    pub attachments: Json,
    pub reactions: Json,
    pub edited_at: Option<ChronoDateTimeUtc>,
    pub deleted_at: Option<ChronoDateTimeUtc>,
    pub metadata: Json,
    pub created_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chat::Entity",
        from = "Column::ChatId",
        to = "super::chat::Column::Id"
    )]
    Chat,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::SenderId",
        to = "super::player::Column::Id"
    )]
    Sender,
}

impl Related<super::chat::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chat.def()
    }
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sender.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
