use crate::{entity::data_type::connector::ConnectorData, value_object::error::AppError};
use async_std::future::Future;

pub trait SearchConnectorRepository: Send + Sync + 'static {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn search(
        &self,
        search_connector_interface: SearchConnectorInterface,
    ) -> impl Future<Output = Result<Vec<ConnectorData>, AppError>> + Send;
}

pub struct SearchConnectorInterface {
    pub keywords: String,
}

impl SearchConnectorInterface {
    pub async fn new(keywords: String) -> Self {
        Self { keywords }
    }
}
