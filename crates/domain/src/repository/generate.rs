use crate::{
    entity::data_type::{generate::GenerateData, generate_data_request::GenerateDataRequest},
    value_object::error::AppError,
};
use async_std::future::Future;

pub trait GenerateRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn generate(
        &self,
        generate_interface: GenerateInterface,
    ) -> impl Future<Output = Result<GenerateData, AppError>> + Send;
}

pub struct GenerateInterface {
    pub generate_data_request: GenerateDataRequest,
}

impl GenerateInterface {
    pub async fn new(generate_data_request: GenerateDataRequest) -> Self {
        Self {
            generate_data_request,
        }
    }
}
