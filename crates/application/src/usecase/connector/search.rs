use domain::{
    entity::data_type::connector::ConnectorData,
    repository::{
        connector::search::{SearchConnectorInterface, SearchConnectorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchConnectorJson {
    pub search_connectors: Vec<ConnectorData>,
}

pub struct SearchConnectorInputs {
    pub keywords: String,
}

pub struct SearchConnectorOutputs<T: HealthCheckRepository, S: SearchConnectorRepository> {
    healyhcheck_repository: T,
    search_connector_repository: S,
}

impl<T: HealthCheckRepository, S: SearchConnectorRepository> SearchConnectorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, search_connector_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            search_connector_repository,
        }
    }
    pub async fn run(
        &self,
        search_connector_inputs: SearchConnectorInputs,
    ) -> Result<SearchConnectorJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let search_connector_interface =
            SearchConnectorInterface::new(search_connector_inputs.keywords).await;
        let result = self
            .search_connector_repository
            .search(search_connector_interface)
            .await?;
        Ok(SearchConnectorJson {
            search_connectors: result,
        })
    }
}
