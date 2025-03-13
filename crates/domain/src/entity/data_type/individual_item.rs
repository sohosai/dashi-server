use chrono::FixedOffset;
use entity::active_enum::Record;
use serde_json::Value;

#[derive(Debug)]
pub struct IndividualItemData {
    pub id: i32,
    pub visible_id: String,
    pub parent_id: i32,
    pub parent_visible_id: String,
    pub record: Record,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Value,
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
