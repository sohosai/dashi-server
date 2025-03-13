use domain::{
    entity::data_type::color::ColorData,
    repository::{
        color::search::{SearchColorInterface, SearchColorRepository},
        healthcheck::HealthCheckRepository,
    },
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchColorJson {
    pub search_colors: Vec<ColorData>,
}

pub struct SearchColorInputs {
    pub keywords: String,
}

pub struct SearchColorOutputs<T: HealthCheckRepository, S: SearchColorRepository> {
    healyhcheck_repository: T,
    search_color_repository: S,
}

impl<T: HealthCheckRepository, S: SearchColorRepository> SearchColorOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, search_color_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            search_color_repository,
        }
    }
    pub async fn run(
        &self,
        search_color_inputs: SearchColorInputs,
    ) -> Result<SearchColorJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let search_color_interface = SearchColorInterface::new(search_color_inputs.keywords).await;
        let result = self
            .search_color_repository
            .search(search_color_interface)
            .await?;
        Ok(SearchColorJson {
            search_colors: result,
        })
    }
}
