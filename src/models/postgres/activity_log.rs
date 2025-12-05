use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "activity_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub action: String,
    pub details: Json,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::ActorId",
        to = "super::player::Column::Id"
    )]
    Actor,
}

impl Related<super::player::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
