use crate::{
    entity::data_type::depreiation_csv::DepreiationCsvData, value_object::error::AppError,
};
use async_std::future::Future;

pub trait DepreiationCsvRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn depreiation_csv(
        &self,
    ) -> impl Future<Output = Result<Vec<DepreiationCsvData>, AppError>> + Send;
}
