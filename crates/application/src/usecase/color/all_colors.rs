use domain::{
    entity::data_type::color::ColorData,
    repository::{color::all_colors::AllColorsRepository, healthcheck::HealthCheckRepository},
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AllColorsJson {
    pub all_colors: Vec<ColorData>,
}

pub struct AllColorsOutputs<T: HealthCheckRepository, S: AllColorsRepository> {
    healyhcheck_repository: T,
    all_colors_repository: S,
}

impl<T: HealthCheckRepository, S: AllColorsRepository> AllColorsOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, all_colors_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            all_colors_repository,
        }
    }
    pub async fn run(&self) -> Result<AllColorsJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let result = self.all_colors_repository.all_colors().await?;
        Ok(AllColorsJson { all_colors: result })
    }
}
