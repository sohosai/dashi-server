use crate::{entity::data_type::connector::ConnectorData, value_object::error::AppError};
use async_std::future::Future;

pub trait AllConnectorsRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn all_connectors(&self) -> impl Future<Output = Result<Vec<ConnectorData>, AppError>> + Send;
}
