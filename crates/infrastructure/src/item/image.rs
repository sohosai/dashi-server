use crate::connection::{self, discord::connect_discord_item_webhook};
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
        let connect_discord_item_webhook = connect_discord_item_webhook().await?;
        image(
            connect_rdb,
            connect_meilisearch,
            image_item_interface.image_item_data,
            connect_discord_item_webhook,
        )
        .await?;
        Ok(())
    }
}
