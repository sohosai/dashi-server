use crate::{entity::data_type::transfer_item::TransferItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait TransferItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn transfer(
        &self,
        transfer_item_interface: TransferItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct TransferItemInterface {
    pub transfer_item_data: TransferItemData,
}

impl TransferItemInterface {
    pub async fn new(transfer_item_data: TransferItemData) -> Self {
        Self { transfer_item_data }
    }
}
