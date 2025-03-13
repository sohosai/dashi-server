use domain::{
    repository::{
        healthcheck::HealthCheckRepository,
        rental::replace::{ReplaceRentalInterface, ReplaceRentalRepository},
    },
    value_object::error::AppError,
};

pub struct ReplaceRentalInputs {
    pub id: u32,
}

pub struct ReplaceRentalOutputs<T: HealthCheckRepository, S: ReplaceRentalRepository> {
    healyhcheck_repository: T,
    replace_rental_repository: S,
}

impl<T: HealthCheckRepository, S: ReplaceRentalRepository> ReplaceRentalOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, replace_rental_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            replace_rental_repository,
        }
    }
    pub async fn run(&self, replace_rental_inputs: ReplaceRentalInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let replace_rental_interface = ReplaceRentalInterface::new(replace_rental_inputs.id).await;
        self.replace_rental_repository
            .replace(replace_rental_interface)
            .await
    }
}
