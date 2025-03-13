use domain::{
    entity::data_type::register_connector::RegisterConnectorData,
    repository::{
        connector::register::{RegisterConnectorInterface, RegisterConnectorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};

pub struct RegisterConnectorInputs {
    pub register_connector_data: RegisterConnectorData,
}

pub struct RegisterConnectorOutputs<T: HealthCheckRepository, S: RegisterConnectorRepository> {
    healyhcheck_repository: T,
    register_connector_repository: S,
}

impl<T: HealthCheckRepository, S: RegisterConnectorRepository> RegisterConnectorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, register_connector_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            register_connector_repository,
        }
    }
    pub async fn run(
        &self,
        register_connector_inputs: RegisterConnectorInputs,
    ) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let register_connector_interface =
            RegisterConnectorInterface::new(register_connector_inputs.register_connector_data)
                .await;
        self.register_connector_repository
            .register(register_connector_interface)
            .await
    }
}
