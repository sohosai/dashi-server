use domain::{
    entity::data_type::item_csv::ItemCsvData,
    repository::{csv::item::ItemCsvRepository, healthcheck::HealthCheckRepository},
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ItemCsvJson {
    pub items: Vec<ItemCsvData>,
}

pub struct ItemCsvOutputs<T: HealthCheckRepository, S: ItemCsvRepository> {
    healyhcheck_repository: T,
    item_csv_repository: S,
}

impl<T: HealthCheckRepository, S: ItemCsvRepository> ItemCsvOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, item_csv_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            item_csv_repository,
        }
    }
    pub async fn run(&self) -> Result<ItemCsvJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        match self.item_csv_repository.item_csv().await {
            Ok(items) => Ok(ItemCsvJson { items }),
            Err(e) => Err(e),
        }
    }
}
