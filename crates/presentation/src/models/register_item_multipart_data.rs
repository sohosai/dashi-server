use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct RegisterItemMultipartData {
    pub parent_visible_id: String,
    pub visible_id: String,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<String>,
    pub purchase_price: Option<String>,
    pub durability: Option<String>,
    pub is_depreciation: String,
    pub connector: String,
    pub color: String,
    #[schema(value_type = String, format = Binary)]
    pub image: Vec<u8>,
}
