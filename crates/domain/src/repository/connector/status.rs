use crate::{
    entity::data_type::status_connector::StatusConnectorData, value_object::error::AppError,
};
use async_std::future::Future;

pub trait StatusConnectorRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn status(
        &self,
        status_connector_interface: StatusConnectorInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct StatusConnectorInterface {
    pub status_connector_data: StatusConnectorData,
}

impl StatusConnectorInterface {
    pub async fn new(status_connector_data: StatusConnectorData) -> Self {
        Self {
            status_connector_data,
        }
    }
}
