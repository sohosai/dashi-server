use domain::{
    entity::data_type::trash_item::TrashItemData,
    repository::{healthcheck::HealthCheckRepository, item::trash::TrashItemRepository},
    value_object::error::AppError,
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TrashItemDataJson {
    pub trash_items: Vec<TrashItemData>,
}

pub struct TrashItemOutputs<T: HealthCheckRepository, S: TrashItemRepository> {
    healyhcheck_repository: T,
    trash_item_repository: S,
}

impl<T: HealthCheckRepository, S: TrashItemRepository> TrashItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, trash_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            trash_item_repository,
        }
    }
    pub async fn run(&self) -> Result<TrashItemDataJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        match self.trash_item_repository.trash().await {
            Ok(result) => Ok(TrashItemDataJson {
                trash_items: result,
            }),
            Err(e) => Err(e),
        }
    }
}
