use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ItemCsvData {
    pub name: String,
    pub product_number: String,
    pub description: String,
}
