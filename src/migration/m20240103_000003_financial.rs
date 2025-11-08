use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Transactions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Transactions::PlayerId).uuid().not_null())
                    .col(ColumnDef::new(Transactions::TransactionType).string().not_null())
                    .col(ColumnDef::new(Transactions::Amount).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Transactions::Currency).string().default("INR"))
                    .col(ColumnDef::new(Transactions::Status).string().default("pending"))
                    .col(ColumnDef::new(Transactions::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_transactions_player")
                            .from(Transactions::Table, Transactions::PlayerId)
                            .to(Players::Table, Players::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    Id,
    PlayerId,
    TransactionType,
    Amount,
    Currency,
    Status,
    CreatedAt,
}

#[derive(Iden)]
enum Players {
    Table,
    Id,
}
