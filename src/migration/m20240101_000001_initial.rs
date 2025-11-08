use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Execute the SQL file content here
        manager
            .create_table(
                Table::create()
                    .table(Players::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Players::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Players::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Players::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Players::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Players::AegisRating).integer().default(1000))
                    .col(ColumnDef::new(Players::Coins).big_integer().default(0))
                    .col(ColumnDef::new(Players::Verified).boolean().default(false))
                    .col(ColumnDef::new(Players::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Players::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Players::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Players {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    AegisRating,
    Coins,
    Verified,
    CreatedAt,
    UpdatedAt,
}
