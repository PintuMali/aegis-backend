use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "team_player_invitations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub team_id: Uuid,
    pub invited_player_id: Uuid,
    pub inviter_id: Uuid,
    pub status: String,
    pub message: Option<String>,
    pub expires_at: ChronoDateTimeUtc,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id"
    )]
    Team,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::InvitedPlayerId",
        to = "super::player::Column::Id"
    )]
    InvitedPlayer,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::InviterId",
        to = "super::player::Column::Id"
    )]
    Inviter,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
