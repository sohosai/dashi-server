use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Record {
    #[sea_orm(string_value = "QR")]
    Qr,
    #[sea_orm(string_value = "Barcode")]
    Barcode,
    #[sea_orm(string_value = "Nothing")]
    Nothing,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum Status {
    #[sea_orm(string_value = "Active")]
    Active,
    #[sea_orm(string_value = "Archive")]
    Archive,
}
