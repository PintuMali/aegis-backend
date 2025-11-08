use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Organizations table should already exist from initial migration
        // Just add missing columns if needed
        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .add_column_if_not_exists(ColumnDef::new(Organizations::ApprovalStatus).string().default("pending"))
                    .add_column_if_not_exists(ColumnDef::new(Organizations::ApprovedBy).uuid())
                    .add_column_if_not_exists(ColumnDef::new(Organizations::ApprovedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organizations::Table)
                    .drop_column(Organizations::ApprovalStatus)
                    .drop_column(Organizations::ApprovedBy)
                    .drop_column(Organizations::ApprovedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Organizations {
    Table,
    ApprovalStatus,
    ApprovedBy,
    ApprovedAt,
}
