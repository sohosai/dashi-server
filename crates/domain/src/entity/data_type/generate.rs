use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize)]
pub struct GenerateData {
    pub visible_ids: Vec<String>,
}
