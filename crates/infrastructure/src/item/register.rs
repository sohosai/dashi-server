use crate::connection::{self, discord::connect_discord_item_webhook};
use crate::item::register::register_module::register;
use domain::{
    repository::{
        connection::ConnectionRepository,
        item::register::{RegisterItemInterface, RegisterItemRepository},
    },
    value_object::error::AppError,
};
use serde::Deserialize;

pub mod register_module;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterItem;

impl RegisterItemRepository for RegisterItem {
    async fn new() -> Self {
        Self {}
    }
    async fn register(&self, register_item_data: RegisterItemInterface) -> Result<(), AppError> {
        let connect_collection = connection::CollectConnection::new().await?;
        let connect_discord_item_webhook = connect_discord_item_webhook().await?;
        register(
            connect_collection.rdb,
            connect_collection.graphdb,
            connect_collection.meilisearch,
            register_item_data.register_item_data,
            connect_discord_item_webhook,
        )
        .await?;
        Ok(())
    }
}
