use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MeilisearchData {
    pub id: i32,
    pub visible_id: String,
    pub record: entity::label::Record,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Vec<String>,
    pub is_rent: bool,
    pub color: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
