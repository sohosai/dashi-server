use crate::connection;
use domain::{
    entity::data_type::trash_item::TrashItemData,
    repository::{connection::ConnectionRepository, item::trash::TrashItemRepository},
    value_object::error::AppError,
};
use trash_module::trash;

pub mod trash_module;

#[derive(Clone)]
pub struct TrashItem;

impl TrashItemRepository for TrashItem {
    async fn new() -> Self {
        Self {}
    }
    async fn trash(&self) -> Result<Vec<TrashItemData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = trash(rdb).await?;
        Ok(result)
    }
}
