use domain::{
    entity::data_type::register_color::RegisterColorData,
    repository::{
        color::register::{RegisterColorInterface, RegisterColorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};

pub struct RegisterColorInputs {
    pub register_color_data: RegisterColorData,
}

pub struct RegisterColorOutputs<T: HealthCheckRepository, S: RegisterColorRepository> {
    healyhcheck_repository: T,
    register_color_repository: S,
}

impl<T: HealthCheckRepository, S: RegisterColorRepository> RegisterColorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, register_color_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            register_color_repository,
        }
    }
    pub async fn run(&self, register_color_inputs: RegisterColorInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let register_color_interface =
            RegisterColorInterface::new(register_color_inputs.register_color_data).await;
        self.register_color_repository
            .register(register_color_interface)
            .await
    }
}
