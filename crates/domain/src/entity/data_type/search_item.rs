use entity::active_enum::Record;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchItemData {
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
    pub created_at: String,
    pub updated_at: String,
    pub recipient: String,
    pub rental_description: String,
    pub latest_rent_at: Option<String>,
    pub scheduled_replace_at: Option<String>,
    pub latest_replace_at: Option<String>,
}
