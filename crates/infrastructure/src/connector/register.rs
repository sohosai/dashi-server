use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        connector::register::{RegisterConnectorInterface, RegisterConnectorRepository},
    },
    value_object::error::AppError,
};
use register_module::register;
use serde::Deserialize;

pub mod register_module;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterConnector;

impl RegisterConnectorRepository for RegisterConnector {
    async fn new() -> Self {
        Self {}
    }
    async fn register(
        &self,
        register_connector_data: RegisterConnectorInterface,
    ) -> Result<(), AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let rdb = connection::CollectConnection::connect_rdb().await?;
        register(
            rdb,
            meilisearch,
            register_connector_data.register_connector_data,
        )
        .await?;
        Ok(())
    }
}
