use domain::{
    entity::data_type::status_connector::StatusConnectorData,
    repository::{
        connector::status::{StatusConnectorInterface, StatusConnectorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};

pub struct StatusConnectorInputs {
    pub status_connector_data: StatusConnectorData,
}

pub struct StatusConnectorOutputs<T: HealthCheckRepository, S: StatusConnectorRepository> {
    healyhcheck_repository: T,
    status_connector_repository: S,
}

impl<T: HealthCheckRepository, S: StatusConnectorRepository> StatusConnectorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, status_connector_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            status_connector_repository,
        }
    }
    pub async fn run(
        &self,
        status_connector_inputs: StatusConnectorInputs,
    ) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let status_connector_interface =
            StatusConnectorInterface::new(status_connector_inputs.status_connector_data).await;
        self.status_connector_repository
            .status(status_connector_interface)
            .await
    }
}
