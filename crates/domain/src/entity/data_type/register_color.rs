use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterColorData {
    pub name: String,
    pub hex_color_code: String,
}
