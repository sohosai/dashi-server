use crate::connection::{self, discord::connect_discord_rental_webhook};
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
        let connect_discord_rental_webhook = connect_discord_rental_webhook().await?;
        update(
            connect_rdb,
            connect_meilisearch,
            update_rental_interface.update_rental_data,
            connect_discord_rental_webhook,
        )
        .await?;
        Ok(())
    }
}
