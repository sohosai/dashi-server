use crate::connection;
use domain::{
    entity::data_type::search_item::SearchItemData,
    repository::{
        connection::ConnectionRepository,
        item::search::{SearchItemInterface, SearchItemRepository},
    },
    value_object::error::AppError,
};
use search_module::search;

pub mod search_module;

#[derive(Clone, Debug)]
pub struct SearchItem;

impl SearchItemRepository for SearchItem {
    async fn new() -> Self {
        Self {}
    }
    async fn search(
        &self,
        search_item_interface: SearchItemInterface,
    ) -> Result<Vec<SearchItemData>, AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let search = search(meilisearch, search_item_interface.keywords).await?;
        Ok(search)
    }
}
