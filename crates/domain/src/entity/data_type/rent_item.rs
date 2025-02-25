use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RentItemData {
    pub item_id: i32,
    pub recipient: String,
    pub description: String,
}
