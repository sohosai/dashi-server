use crate::connection::{self, discord::connect_discord_item_webhook};
use domain::{
    repository::{
        connection::ConnectionRepository,
        item::transfer::{TransferItemInterface, TransferItemRepository},
    },
    value_object::error::AppError,
};
use transfer_module::transfer;

pub mod transfer_module;

#[derive(Clone, Debug)]
pub struct TransferItem;

impl TransferItemRepository for TransferItem {
    async fn new() -> Self {
        Self {}
    }
    async fn transfer(&self, transfer_item_data: TransferItemInterface) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_graphdb = connection::CollectConnection::connect_graphdb().await?;
        let connect_discord_item_webhook = connect_discord_item_webhook().await?;
        transfer(
            connect_rdb,
            connect_graphdb,
            transfer_item_data.transfer_item_data,
            connect_discord_item_webhook,
        )
        .await?;
        Ok(())
    }
}
