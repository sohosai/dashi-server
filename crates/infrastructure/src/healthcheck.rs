use crate::connection;
use domain::{
    repository::{connection::ConnectionRepository, healthcheck::HealthCheckRepository},
    value_object::error::healthcheck::HealthCheckError,
};
use graphdb::healthcheck_graphdb;
use meilisearch::healthcheck_meilisearch;
use rdb::healthcheck_rdb;

pub mod graphdb;
pub mod meilisearch;
pub mod rdb;

#[derive(Clone)]
pub struct HealthCheck;

impl HealthCheckRepository for HealthCheck {
    async fn new() -> Self {
        Self {}
    }
    async fn healthcheck(&self) -> Result<(), HealthCheckError> {
        let connect_collections = connection::CollectConnection::new().await?;
        healthcheck_graphdb(connect_collections.graphdb).await?;
        healthcheck_meilisearch(connect_collections.meilisearch).await?;
        healthcheck_rdb(connect_collections.rdb).await?;
        Ok(())
    }
}
