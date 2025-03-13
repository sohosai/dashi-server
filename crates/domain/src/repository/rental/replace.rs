use crate::value_object::error::AppError;
use async_std::future::Future;

pub trait ReplaceRentalRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn replace(
        &self,
        replace_rental_interface: ReplaceRentalInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct ReplaceRentalInterface {
    pub id: u32,
}

impl ReplaceRentalInterface {
    pub async fn new(id: u32) -> Self {
        Self { id }
    }
}
