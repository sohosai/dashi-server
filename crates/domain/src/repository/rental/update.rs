use crate::{entity::data_type::rental::RentalData, value_object::error::AppError};
use async_std::future::Future;

pub trait UpdateRentalRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn update(
        &self,
        update_rental_interface: UpdateRentalInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct UpdateRentalInterface {
    pub update_rental_data: RentalData,
}

impl UpdateRentalInterface {
    pub async fn new(update_rental_data: RentalData) -> Self {
        Self { update_rental_data }
    }
}
