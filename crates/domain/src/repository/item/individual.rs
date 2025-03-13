use crate::{
    entity::data_type::individual_item::IndividualItemData, value_object::error::AppError,
};
use async_std::future::Future;

pub trait IndividualItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn individual(
        &self,
        individual_item_interface: IndividualItemInterface,
    ) -> impl Future<Output = Result<IndividualItemData, AppError>> + Send;
}

pub struct IndividualItemInterface {
    pub id: u32,
}

impl IndividualItemInterface {
    pub async fn new(id: u32) -> Self {
        Self { id }
    }
}
