use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateDataRequest {
    pub quantity: u32,
    pub record: String,
}
