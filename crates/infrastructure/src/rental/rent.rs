use crate::connection::{self, discord::connect_discord_rental_webhook};
use domain::{
    repository::{
        connection::ConnectionRepository,
        rental::rent::{RentRentalInterface, RentRentalRepository},
    },
    value_object::error::AppError,
};
use rent_module::rent;

pub mod rent_module;

#[derive(Clone, Debug)]
pub struct RentRental;

impl RentRentalRepository for RentRental {
    async fn new() -> Self {
        Self {}
    }
    async fn rent(&self, rent_rental_interface: RentRentalInterface) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let connect_discord_rental_webhook = connect_discord_rental_webhook().await?;
        rent(
            connect_rdb,
            connect_meilisearch,
            rent_rental_interface.rent_rental_data,
            connect_discord_rental_webhook,
        )
        .await?;
        Ok(())
    }
}
