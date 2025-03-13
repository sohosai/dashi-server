use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        item::image::{ImageItemInterface, ImageItemRepository},
    },
    value_object::error::AppError,
};
use image_module::image;

pub mod image_module;

#[derive(Clone, Debug)]
pub struct ImageItem;

impl ImageItemRepository for ImageItem {
    async fn new() -> Self {
        Self {}
    }
    async fn image(&self, image_item_interface: ImageItemInterface) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        image(
            connect_rdb,
            connect_meilisearch,
            image_item_interface.image_item_data,
        )
        .await?;
        Ok(())
    }
}
