use domain::{
    entity::data_type::individual_item::IndividualItemData,
    repository::{
        connection::ConnectionRepository,
        item::individual::{IndividualItemInterface, IndividualItemRepository},
    },
    value_object::error::AppError,
};
use individual_item_module::individual_item;

use crate::connection;
pub mod individual_item_module;

#[derive(Clone, Debug)]
pub struct IndividualItem;

impl IndividualItemRepository for IndividualItem {
    async fn new() -> Self {
        Self {}
    }
    async fn individual(
        &self,
        individual_item_interface: IndividualItemInterface,
    ) -> Result<IndividualItemData, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let graphdb = connection::CollectConnection::connect_graphdb().await?;
        let result = individual_item(rdb, graphdb, individual_item_interface.id).await?;
        Ok(result)
    }
}
