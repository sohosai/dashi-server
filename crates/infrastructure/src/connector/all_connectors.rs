use crate::connection;
use all_connectors_module::all_connectors;
use domain::{
    entity::data_type::connector::ConnectorData,
    repository::{
        connection::ConnectionRepository, connector::all_connectors::AllConnectorsRepository,
    },
    value_object::error::AppError,
};
use serde::Deserialize;

pub mod all_connectors_module;

#[derive(Clone, Debug, Deserialize)]
pub struct AllConnectors;

impl AllConnectorsRepository for AllConnectors {
    async fn new() -> Self {
        Self {}
    }
    async fn all_connectors(&self) -> Result<Vec<ConnectorData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = all_connectors(rdb).await?;
        Ok(result)
    }
}
