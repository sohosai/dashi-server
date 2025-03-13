use chrono::FixedOffset;
use entity::active_enum::Record;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeilisearchItemData {
    pub id: i32,
    pub visible_id: String,
    pub record: Record,
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
    pub recipient: String,
    pub rental_description: String,
    pub latest_rent_at: Option<chrono::NaiveDateTime>,
    pub scheduled_replace_at: Option<chrono::DateTime<FixedOffset>>,
    pub latest_replace_at: Option<chrono::NaiveDateTime>,
}
