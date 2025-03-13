use crate::m20220101_000001_label_table::Label;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Trash::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Trash::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Trash::ItemId).integer().not_null())
                    .col(ColumnDef::new(Trash::VisibleId).string().not_null())
                    .col(ColumnDef::new(Trash::Name).string().not_null())
                    .col(
                        ColumnDef::new(Trash::ProductNumber)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Trash::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Trash::PurchaseYear).integer())
                    .col(ColumnDef::new(Trash::PurchasePrice).integer())
                    .col(ColumnDef::new(Trash::Durability).integer())
                    .col(ColumnDef::new(Trash::IsDepreciation).boolean().not_null())
                    .col(ColumnDef::new(Trash::Connector).json().not_null())
                    .col(ColumnDef::new(Trash::IsRent).boolean().not_null())
                    .col(ColumnDef::new(Trash::Color).string().not_null().default(""))
                    .col(ColumnDef::new(Trash::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Trash::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Trash::Table, Trash::VisibleId)
                            .to(Label::Table, Label::VisibleId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Trash::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Trash {
    Table,
    Id,
    ItemId,
    VisibleId,
    Name,
    ProductNumber,
    Description,
    PurchaseYear,
    PurchasePrice,
    Durability,
    IsDepreciation,
    Connector,
    IsRent,
    Color,
    CreatedAt,
    UpdatedAt,
}
