use crate::{entity::data_type::item_csv::ItemCsvData, value_object::error::AppError};
use async_std::future::Future;

pub trait ItemCsvRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn item_csv(&self) -> impl Future<Output = Result<Vec<ItemCsvData>, AppError>> + Send;
}
