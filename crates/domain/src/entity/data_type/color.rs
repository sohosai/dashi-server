use entity::active_enum::Status;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ColorData {
    pub id: i32,
    pub name: String,
    pub hex_color_code: String,
    pub status: Status,
}
