use crate::connection;
use domain::{
    entity::data_type::item_csv::ItemCsvData,
    repository::{connection::ConnectionRepository, csv::item::ItemCsvRepository},
    value_object::error::AppError,
};
use item_module::item;

pub mod item_module;

#[derive(Clone)]
pub struct ItemCsv;

impl ItemCsvRepository for ItemCsv {
    async fn new() -> Self {
        Self {}
    }
    async fn item_csv(&self) -> Result<Vec<ItemCsvData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = item(rdb).await?;
        Ok(result)
    }
}
