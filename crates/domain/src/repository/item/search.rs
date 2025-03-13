use crate::{entity::data_type::search_item::SearchItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait SearchItemRepository: Send + Sync + 'static {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn search(
        &self,
        search_item_interface: SearchItemInterface,
    ) -> impl Future<Output = Result<Vec<SearchItemData>, AppError>> + Send;
}

pub struct SearchItemInterface {
    pub keywords: String,
}

impl SearchItemInterface {
    pub async fn new(keywords: String) -> Self {
        Self { keywords }
    }
}
