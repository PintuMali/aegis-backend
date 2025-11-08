use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerGameStats::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PlayerGameStats::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(PlayerGameStats::PlayerId).uuid().not_null())
                    .col(ColumnDef::new(PlayerGameStats::GameType).string().not_null())
                    .col(ColumnDef::new(PlayerGameStats::MatchesPlayed).integer().default(0))
                    .col(ColumnDef::new(PlayerGameStats::Wins).integer().default(0))
                    .col(ColumnDef::new(PlayerGameStats::Kills).integer().default(0))
                    .col(ColumnDef::new(PlayerGameStats::Stats).json())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_stats_player")
                            .from(PlayerGameStats::Table, PlayerGameStats::PlayerId)
                            .to(Players::Table, Players::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TournamentTeams::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TournamentTeams::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(TournamentTeams::TournamentId).uuid().not_null())
                    .col(ColumnDef::new(TournamentTeams::TeamId).uuid().not_null())
                    .col(ColumnDef::new(TournamentTeams::TotalPoints).integer().default(0))
                    .col(ColumnDef::new(TournamentTeams::TotalKills).integer().default(0))
                    .col(ColumnDef::new(TournamentTeams::JoinedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tournament_teams_tournament")
                            .from(TournamentTeams::Table, TournamentTeams::TournamentId)
                            .to(Tournaments::Table, Tournaments::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tournament_teams_team")
                            .from(TournamentTeams::Table, TournamentTeams::TeamId)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TournamentTeams::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PlayerGameStats::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PlayerGameStats {
    Table,
    Id,
    PlayerId,
    GameType,
    MatchesPlayed,
    Wins,
    Kills,
    Stats,
}

#[derive(Iden)]
enum TournamentTeams {
    Table,
    Id,
    TournamentId,
    TeamId,
    TotalPoints,
    TotalKills,
    JoinedAt,
}

#[derive(Iden)]
enum Players {
    Table,
    Id,
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
