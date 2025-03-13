use entity::active_enum::Status;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateColorData {
    pub id: i32,
    pub hex_color_code: String,
    pub status: Status,
}
