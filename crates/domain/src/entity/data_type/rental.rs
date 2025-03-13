use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RentalData {
    pub id: i32,
    pub recipient: String,
    pub rental_description: String,
    pub scheduled_replace_at: String,
}
