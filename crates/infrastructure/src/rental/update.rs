use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        rental::update::{UpdateRentalInterface, UpdateRentalRepository},
    },
    value_object::error::AppError,
};
use update_module::update;

pub mod update_module;

#[derive(Clone, Debug)]
pub struct UpdateRental;

impl UpdateRentalRepository for UpdateRental {
    async fn new() -> Self {
        Self {}
    }
    async fn update(&self, update_rental_interface: UpdateRentalInterface) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        update(
            connect_rdb,
            connect_meilisearch,
            update_rental_interface.update_rental_data,
        )
        .await?;
        Ok(())
    }
}
