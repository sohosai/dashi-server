use domain::{
    entity::data_type::rental::RentalData,
    repository::{
        healthcheck::HealthCheckRepository,
        rental::rent::{RentRentalInterface, RentRentalRepository},
    },
    value_object::error::AppError,
};

pub struct RentRentalInputs {
    pub rental_data: RentalData,
}

pub struct RentRentalOutputs<T: HealthCheckRepository, S: RentRentalRepository> {
    healyhcheck_repository: T,
    rent_rental_repository: S,
}

impl<T: HealthCheckRepository, S: RentRentalRepository> RentRentalOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, rent_rental_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            rent_rental_repository,
        }
    }
    pub async fn run(&self, rent_rental_inputs: RentRentalInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let rent_rental_interface = RentRentalInterface::new(rent_rental_inputs.rental_data).await;
        self.rent_rental_repository
            .rent(rent_rental_interface)
            .await
    }
}
