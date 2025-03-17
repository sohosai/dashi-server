use crate::{entity::data_type::rental_item::RentalItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait AllRentalItemsRepository: Send + Sync + 'static {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn all_rental_items(
        &self,
    ) -> impl Future<Output = Result<Vec<RentalItemData>, AppError>> + Send;
}

pub struct AllRentalItemsInterface;

impl AllRentalItemsInterface {
    pub async fn new() -> Self {
        Self {}
    }
}
