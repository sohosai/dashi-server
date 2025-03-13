use crate::connection;
use domain::{
    entity::data_type::connector::ConnectorData,
    repository::{
        connection::ConnectionRepository,
        connector::search::{SearchConnectorInterface, SearchConnectorRepository},
    },
    value_object::error::AppError,
};
use search_module::search;
use serde::Deserialize;

pub mod search_module;

#[derive(Clone, Debug, Deserialize)]
pub struct SearchConnector;

impl SearchConnectorRepository for SearchConnector {
    async fn new() -> Self {
        Self {}
    }
    async fn search(
        &self,
        search_connector_data: SearchConnectorInterface,
    ) -> Result<Vec<ConnectorData>, AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let result = search(meilisearch, search_connector_data.keywords).await?;
        Ok(result)
    }
}
