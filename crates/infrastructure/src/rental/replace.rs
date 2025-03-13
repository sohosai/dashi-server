use crate::connection;
use domain::{
    repository::{
        connection::ConnectionRepository,
        rental::replace::{ReplaceRentalInterface, ReplaceRentalRepository},
    },
    value_object::error::AppError,
};
use replace_module::replace;

pub mod replace_module;

#[derive(Clone, Debug)]
pub struct ReplaceRental;

impl ReplaceRentalRepository for ReplaceRental {
    async fn new() -> Self {
        Self {}
    }
    async fn replace(
        &self,
        replace_rental_interface: ReplaceRentalInterface,
    ) -> Result<(), AppError> {
        let connect_rdb = connection::CollectConnection::connect_rdb().await?;
        let connect_meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        replace(
            connect_rdb,
            connect_meilisearch,
            replace_rental_interface.id,
        )
        .await?;
        Ok(())
    }
}
