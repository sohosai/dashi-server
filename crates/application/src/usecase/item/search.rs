use domain::{
    entity::data_type::search_item::SearchItemData,
    repository::{
        healthcheck::HealthCheckRepository,
        item::search::{SearchItemInterface, SearchItemRepository},
    },
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchItemJson {
    pub search_items: Vec<SearchItemData>,
}

pub struct SearchItemInputs {
    pub keywords: String,
}

pub struct SearchItemOutputs<T: HealthCheckRepository, S: SearchItemRepository> {
    healyhcheck_repository: T,
    search_item_repository: S,
}

impl<T: HealthCheckRepository, S: SearchItemRepository> SearchItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, search_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            search_item_repository,
        }
    }
    pub async fn run(
        &self,
        search_item_inputs: SearchItemInputs,
    ) -> Result<Vec<SearchItemData>, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let search_item_interface = SearchItemInterface::new(search_item_inputs.keywords).await;
        self.search_item_repository
            .search(search_item_interface)
            .await
    }
}
