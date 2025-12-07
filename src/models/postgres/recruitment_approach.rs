// src/models/postgres/recruitment_approach.rs
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "recruitment_approaches")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub team_id: Uuid,
    pub recruiter_id: Uuid,
    pub target_player_id: Uuid,
    pub status: String, // pending, accepted, declined, withdrawn
    pub message: Option<String>,
    pub position_offered: Option<String>,
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
        from = "Column::RecruiterId",
        to = "super::player::Column::Id"
    )]
    Recruiter,
    #[sea_orm(
        belongs_to = "super::player::Entity",
        from = "Column::TargetPlayerId",
        to = "super::player::Column::Id"
    )]
    TargetPlayer,
}

impl ActiveModelBehavior for ActiveModel {}
