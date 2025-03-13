use crate::{entity::data_type::update_item::UpdateItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait UpdateItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn update(
        &self,
        update_item_interface: UpdateItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct UpdateItemInterface {
    pub update_item_data: UpdateItemData,
}

impl UpdateItemInterface {
    pub async fn new(update_item_data: UpdateItemData) -> Self {
        Self { update_item_data }
    }
}
