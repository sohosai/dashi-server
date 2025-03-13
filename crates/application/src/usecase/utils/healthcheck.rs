use domain::{
    repository::healthcheck::HealthCheckRepository,
    value_object::error::healthcheck::HealthCheckError,
};

pub struct HealthCheckUseCase<T: HealthCheckRepository> {
    healthcheck_interface: T,
}

impl<T: HealthCheckRepository> HealthCheckUseCase<T> {
    pub async fn new(healthcheck_interface: T) -> Self {
        Self {
            healthcheck_interface,
        }
    }
    pub async fn run(&self) -> Result<(), HealthCheckError> {
        self.healthcheck_interface.healthcheck().await
    }
}
