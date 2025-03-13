use crate::{
    entity::data_type::register_connector::RegisterConnectorData, value_object::error::AppError,
};
use async_std::future::Future;

pub trait RegisterConnectorRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn register(
        &self,
        register_connector_interface: RegisterConnectorInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct RegisterConnectorInterface {
    pub register_connector_data: RegisterConnectorData,
}

impl RegisterConnectorInterface {
    pub async fn new(register_connector_data: RegisterConnectorData) -> Self {
        Self {
            register_connector_data,
        }
    }
}
