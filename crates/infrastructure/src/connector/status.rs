use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        connector::status::{StatusConnectorInterface, StatusConnectorRepository},
    },
    value_object::error::AppError,
};
use serde::Deserialize;
use status_module::status;

pub mod status_module;

#[derive(Clone, Debug, Deserialize)]
pub struct StatusConnector;

impl StatusConnectorRepository for StatusConnector {
    async fn new() -> Self {
        Self {}
    }
    async fn status(
        &self,
        status_connector_data: StatusConnectorInterface,
    ) -> Result<(), AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let rdb = connection::CollectConnection::connect_rdb().await?;
        status(
            rdb,
            meilisearch,
            status_connector_data.status_connector_data,
        )
        .await?;
        Ok(())
    }
}
