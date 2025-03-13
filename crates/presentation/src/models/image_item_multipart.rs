use utoipa::ToSchema;

#[derive(Debug, ToSchema)]
pub struct ImageItemMultipartData {
    #[schema(value_type = String, format = Binary)]
    pub image: Vec<u8>,
}
