use domain::{
    entity::data_type::{generate::GenerateData, generate_data_request::GenerateDataRequest},
    repository::{
        generate::{GenerateInterface, GenerateRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};

pub struct GenerateInputs {
    pub generate_data: GenerateDataRequest,
}

pub struct GenerateOutputs<T: HealthCheckRepository, S: GenerateRepository> {
    healthcheck_repository: T,
    generate_repository: S,
}

impl<T: HealthCheckRepository, S: GenerateRepository> GenerateOutputs<T, S> {
    pub async fn new(healthcheck_repository: T, generate_repository: S) -> Self {
        Self {
            healthcheck_repository,
            generate_repository,
        }
    }
    pub async fn run(&self, generate_inputs: GenerateInputs) -> Result<GenerateData, AppError> {
        self.healthcheck_repository.healthcheck().await?;
        let generate_interface = GenerateInterface::new(generate_inputs.generate_data).await;
        self.generate_repository.generate(generate_interface).await
    }
}
