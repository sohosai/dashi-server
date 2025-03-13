use axum::body::Bytes;
use domain::{
    entity::data_type::image_item::ImageItemData,
    repository::{
        healthcheck::HealthCheckRepository,
        item::image::{ImageItemInterface, ImageItemRepository},
    },
    value_object::error::AppError,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImageItemDataJson {
    pub image: Bytes,
}

impl Default for ImageItemDataJson {
    fn default() -> Self {
        Self {
            image: Bytes::new(),
        }
    }
}

pub struct ImageItemInputs {
    pub id: u32,
    pub image_item_data_json: ImageItemDataJson,
}

pub struct ImageItemOutputs<T: HealthCheckRepository, S: ImageItemRepository> {
    healyhcheck_repository: T,
    image_item_repository: S,
}

impl<T: HealthCheckRepository, S: ImageItemRepository> ImageItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, image_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            image_item_repository,
        }
    }
    pub async fn run(&self, image_item_inputs: ImageItemInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let image_item_data = ImageItemData {
            id: image_item_inputs.id as i32,
            image: image_item_inputs.image_item_data_json.image,
        };
        let image_item_interface = ImageItemInterface::new(image_item_data).await;
        self.image_item_repository.image(image_item_interface).await
    }
}
