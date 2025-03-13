use axum::body::Bytes;

#[derive(Clone, Debug)]
pub struct RegisterItemData {
    pub parent_visible_id: String,
    pub visible_id: String,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Vec<String>,
    pub color: String,
    pub image: Bytes,
}

impl Default for RegisterItemData {
    fn default() -> Self {
        Self {
            parent_visible_id: "".to_string(),
            visible_id: "".to_string(),
            name: "".to_string(),
            product_number: "".to_string(),
            description: "".to_string(),
            purchase_year: None,
            purchase_price: None,
            durability: None,
            is_depreciation: false,
            connector: Vec::new(),
            color: "".to_string(),
            image: Bytes::new(),
        }
    }
}
