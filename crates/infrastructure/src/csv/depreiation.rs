use crate::connection;
use depreiation_module::depreiation;
use domain::{
    entity::data_type::depreiation_csv::DepreiationCsvData,
    repository::{connection::ConnectionRepository, csv::depreiation::DepreiationCsvRepository},
    value_object::error::AppError,
};

pub mod depreiation_module;

#[derive(Clone)]
pub struct DepreiationCsv;

impl DepreiationCsvRepository for DepreiationCsv {
    async fn new() -> Self {
        Self {}
    }
    async fn depreiation_csv(&self) -> Result<Vec<DepreiationCsvData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = depreiation(rdb).await?;
        Ok(result)
    }
}
