use crate::connection;
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
        let graphdb = connection::CollectConnection::connect_graphdb().await?;
        transfer(graphdb, transfer_item_data.transfer_item_data).await?;
        Ok(())
    }
}
