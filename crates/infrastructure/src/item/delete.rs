use crate::connection::{self, discord::connect_discord_item_webhook};
use delete_module::delete;
use domain::{
    repository::{
        connection::ConnectionRepository,
        item::delete::{DeleteItemInterface, DeleteItemRepository},
    },
    value_object::error::AppError,
};

pub mod delete_module;

#[derive(Clone, Debug)]
pub struct DeleteItem;

impl DeleteItemRepository for DeleteItem {
    async fn new() -> Self {
        Self {}
    }
    async fn delete(&self, delete_item_interface: DeleteItemInterface) -> Result<(), AppError> {
        let connect_collection = connection::CollectConnection::new().await?;
        let connect_discord_item_webhook = connect_discord_item_webhook().await?;
        delete(
            connect_collection.rdb,
            connect_collection.graphdb,
            connect_collection.meilisearch,
            delete_item_interface.id,
            connect_discord_item_webhook,
        )
        .await?;
        Ok(())
    }
}
