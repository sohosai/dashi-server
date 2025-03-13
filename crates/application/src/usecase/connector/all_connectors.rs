use domain::{
    entity::data_type::connector::ConnectorData,
    repository::{
        connector::all_connectors::AllConnectorsRepository, healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AllConnectorsJson {
    pub all_connectors: Vec<ConnectorData>,
}

pub struct AllConnectorsOutputs<T: HealthCheckRepository, S: AllConnectorsRepository> {
    healyhcheck_repository: T,
    all_connectors_repository: S,
}

impl<T: HealthCheckRepository, S: AllConnectorsRepository> AllConnectorsOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, all_connectors_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            all_connectors_repository,
        }
    }
    pub async fn run(&self) -> Result<AllConnectorsJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let result = self.all_connectors_repository.all_connectors().await?;
        Ok(AllConnectorsJson {
            all_connectors: result,
        })
    }
}
