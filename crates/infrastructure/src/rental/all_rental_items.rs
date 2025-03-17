use crate::connection;
use all_rental_items_module::all_rental_items;
use domain::{
    entity::data_type::rental_item::RentalItemData,
    repository::{
        connection::ConnectionRepository, rental::all_rental_items::AllRentalItemsRepository,
    },
    value_object::error::AppError,
};

pub mod all_rental_items_module;

#[derive(Clone, Debug)]
pub struct AllRentalItems;

impl AllRentalItemsRepository for AllRentalItems {
    async fn new() -> Self {
        Self {}
    }
    async fn all_rental_items(&self) -> Result<Vec<RentalItemData>, AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let all_rental_items = all_rental_items(meilisearch).await?;
        Ok(all_rental_items)
    }
}
