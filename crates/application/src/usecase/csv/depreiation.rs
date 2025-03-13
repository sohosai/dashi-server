use domain::{
    entity::data_type::depreiation_csv::DepreiationCsvData,
    repository::{csv::depreiation::DepreiationCsvRepository, healthcheck::HealthCheckRepository},
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct DepreiationCsvJson {
    pub depreciation_items: Vec<DepreiationCsvData>,
}

pub struct DepreiationCsvOutputs<T: HealthCheckRepository, S: DepreiationCsvRepository> {
    healyhcheck_repository: T,
    depreiation_csv_repository: S,
}

impl<T: HealthCheckRepository, S: DepreiationCsvRepository> DepreiationCsvOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, depreiation_csv_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            depreiation_csv_repository,
        }
    }
    pub async fn run(&self) -> Result<DepreiationCsvJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        match self.depreiation_csv_repository.depreiation_csv().await {
            Ok(depreciation_items) => Ok(DepreiationCsvJson { depreciation_items }),
            Err(e) => Err(e),
        }
    }
}
