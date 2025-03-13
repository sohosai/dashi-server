use crate::{entity::data_type::rental::RentalData, value_object::error::AppError};
use async_std::future::Future;

pub trait RentRentalRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn rent(
        &self,
        rent_rental_interface: RentRentalInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct RentRentalInterface {
    pub rent_rental_data: RentalData,
}

impl RentRentalInterface {
    pub async fn new(rent_rental_data: RentalData) -> Self {
        Self { rent_rental_data }
    }
}
