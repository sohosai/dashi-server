use entity::active_enum::Status;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ConnectorData {
    pub id: i32,
    pub name: String,
    pub status: Status,
}
