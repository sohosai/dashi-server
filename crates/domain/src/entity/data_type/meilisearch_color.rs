use entity::active_enum::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MeilisearchColorData {
    pub id: i32,
    pub name: String,
    pub hex_color_code: String,
    pub status: Status,
}
