use entity::active_enum::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MeilisearchConnectorData {
    pub id: i32,
    pub name: String,
    pub status: Status,
}
