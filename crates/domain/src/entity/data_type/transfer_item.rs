use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TransferItemData {
    pub id: u32,
    pub new_parent_id: u32,
}
