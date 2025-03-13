use domain::{
    entity::data_type::rental::RentalData,
    repository::{
        healthcheck::HealthCheckRepository,
        rental::update::{UpdateRentalInterface, UpdateRentalRepository},
    },
    value_object::error::AppError,
};

pub struct UpdateRentalInputs {
    pub rental_data: RentalData,
}

pub struct UpdateRentalOutputs<T: HealthCheckRepository, S: UpdateRentalRepository> {
    healyhcheck_repository: T,
    update_rental_repository: S,
}

impl<T: HealthCheckRepository, S: UpdateRentalRepository> UpdateRentalOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, update_rental_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            update_rental_repository,
        }
    }
    pub async fn run(&self, update_rental_inputs: UpdateRentalInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let update_rental_interface =
            UpdateRentalInterface::new(update_rental_inputs.rental_data).await;
        self.update_rental_repository
            .update(update_rental_interface)
            .await
    }
}
