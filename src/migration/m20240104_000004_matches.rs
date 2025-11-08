use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create matches table
        manager
            .create_table(
                Table::create()
                    .table(Matches::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Matches::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Matches::TournamentId).uuid().not_null())
                    .col(ColumnDef::new(Matches::MatchNumber).integer().not_null())
                    .col(ColumnDef::new(Matches::GameType).string().not_null())
                    .col(ColumnDef::new(Matches::Status).string().default("scheduled"))
                    .col(ColumnDef::new(Matches::MapName).string())
                    .col(ColumnDef::new(Matches::ScheduledStart).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Matches::ActualStart).timestamp_with_time_zone())
                    .col(ColumnDef::new(Matches::ActualEnd).timestamp_with_time_zone())
                    .col(ColumnDef::new(Matches::Settings).json())
                    .col(ColumnDef::new(Matches::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_matches_tournament")
                            .from(Matches::Table, Matches::TournamentId)
                            .to(Tournaments::Table, Tournaments::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create match participants table
        manager
            .create_table(
                Table::create()
                    .table(MatchParticipants::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MatchParticipants::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(MatchParticipants::MatchId).uuid().not_null())
                    .col(ColumnDef::new(MatchParticipants::TeamId).uuid().not_null())
                    .col(ColumnDef::new(MatchParticipants::Placement).integer())
                    .col(ColumnDef::new(MatchParticipants::Kills).integer().default(0))
                    .col(ColumnDef::new(MatchParticipants::Points).integer().default(0))
                    .col(ColumnDef::new(MatchParticipants::Stats).json())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_match_participants_match")
                            .from(MatchParticipants::Table, MatchParticipants::MatchId)
                            .to(Matches::Table, Matches::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_match_participants_team")
                            .from(MatchParticipants::Table, MatchParticipants::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MatchParticipants::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Matches::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Matches {
    Table,
    Id,
    TournamentId,
    MatchNumber,
    GameType,
    Status,
    MapName,
    ScheduledStart,
    ActualStart,
    ActualEnd,
    Settings,
    CreatedAt,
}

#[derive(Iden)]
enum MatchParticipants {
    Table,
    Id,
    MatchId,
    TeamId,
    Placement,
    Kills,
    Points,
    Stats,
}

#[derive(Iden)]
enum Tournaments {
    Table,
    Id,
}

#[derive(Iden)]
enum Teams {
    Table,
    Id,
}
