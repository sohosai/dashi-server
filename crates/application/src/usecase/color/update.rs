use domain::{
    entity::data_type::update_color::UpdateColorData,
    repository::{
        color::update::{UpdateColorInterface, UpdateColorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};

pub struct UpdateColorInputs {
    pub update_color_data: UpdateColorData,
}

pub struct UpdateColorOutputs<T: HealthCheckRepository, S: UpdateColorRepository> {
    healyhcheck_repository: T,
    update_color_repository: S,
}

impl<T: HealthCheckRepository, S: UpdateColorRepository> UpdateColorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, update_color_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            update_color_repository,
        }
    }
    pub async fn run(&self, update_color_inputs: UpdateColorInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let update_color_interface =
            UpdateColorInterface::new(update_color_inputs.update_color_data).await;
        self.update_color_repository
            .update(update_color_interface)
            .await
    }
}
