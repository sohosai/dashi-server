use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        item::update::{UpdateItemInterface, UpdateItemRepository},
    },
    value_object::error::AppError,
};
use update_module::update;

pub mod update_module;

#[derive(Clone, Debug)]
pub struct UpdateItem;

impl UpdateItemRepository for UpdateItem {
    async fn new() -> Self {
        Self {}
    }
    async fn update(&self, update_item_interface: UpdateItemInterface) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        update(
            connect_rdb,
            connect_meilisearch,
            update_item_interface.update_item_data,
        )
        .await?;
        Ok(())
    }
}
