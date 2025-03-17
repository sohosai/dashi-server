use domain::{
    entity::data_type::rental_item::RentalItemData,
    repository::{
        healthcheck::HealthCheckRepository, rental::all_rental_items::AllRentalItemsRepository,
    },
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RentalItemJson {
    pub rental_items: Vec<RentalItemData>,
}

pub struct AllRentalItemsOutputs<T: HealthCheckRepository, S: AllRentalItemsRepository> {
    healyhcheck_repository: T,
    all_rental_items_repository: S,
}

impl<T: HealthCheckRepository, S: AllRentalItemsRepository> AllRentalItemsOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, all_rental_items_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            all_rental_items_repository,
        }
    }
    pub async fn run(&self) -> Result<Vec<RentalItemData>, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        self.all_rental_items_repository.all_rental_items().await
    }
}
