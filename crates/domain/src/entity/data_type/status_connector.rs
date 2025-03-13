use entity::active_enum::Status;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct StatusConnectorData {
    pub id: i32,
    pub status: Status,
}
