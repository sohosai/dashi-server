use crate::value_object::error::AppError;
use async_std::future::Future;

pub trait DeleteItemRepository: Send + Sync + 'static {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn delete(
        &self,
        delete_item_interface: DeleteItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct DeleteItemInterface {
    pub id: u32,
}

impl DeleteItemInterface {
    pub async fn new(id: u32) -> Self {
        Self { id }
    }
}
