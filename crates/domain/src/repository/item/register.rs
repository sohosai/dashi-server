use crate::{entity::data_type::register_item::RegisterItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait RegisterItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn register(
        &self,
        register_item_interface: RegisterItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct RegisterItemInterface {
    pub register_item_data: RegisterItemData,
}

impl RegisterItemInterface {
    pub async fn new(register_item_data: RegisterItemData) -> Self {
        Self { register_item_data }
    }
}
