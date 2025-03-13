use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Color::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Color::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Color::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Color::HexColorCode)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Color::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Color::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Color {
    Table,
    Id,
    Name,
    HexColorCode,
    Status,
}
