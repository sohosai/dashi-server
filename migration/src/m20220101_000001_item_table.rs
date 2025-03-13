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
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Item::VisibleId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(
                        ColumnDef::new(Item::ProductNumber)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Item::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Item::PurchaseYear).integer())
                    .col(ColumnDef::new(Item::PurchasePrice).integer())
                    .col(ColumnDef::new(Item::Durability).integer())
                    .col(ColumnDef::new(Item::IsDepreciation).boolean().not_null())
                    .col(ColumnDef::new(Item::Connector).json().not_null())
                    .col(ColumnDef::new(Item::IsRent).boolean().not_null())
                    .col(ColumnDef::new(Item::Color).string().not_null().default(""))
                    .col(ColumnDef::new(Item::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Item::UpdatedAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(Item::Recipient)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Item::RentalDescription)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Item::LatestRentAt).timestamp())
                    .col(ColumnDef::new(Item::ScheduledReplaceAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Item::LatestReplaceAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::VisibleId)
                            .to(Label::Table, Label::VisibleId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Item {
    Table,
    Id,
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
    Recipient,
    RentalDescription,
    LatestRentAt,
    ScheduledReplaceAt,
    LatestReplaceAt,
}
