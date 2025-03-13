use crate::{entity::data_type::trash_item::TrashItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait TrashItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn trash(&self) -> impl Future<Output = Result<Vec<TrashItemData>, AppError>> + Send;
}
