use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitializeError {
    #[error(transparent)]
    InsertCsvDataError(#[from] crate::error::csv::InsertCsvDataError),
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    ConnectionError(#[from] domain::value_object::error::connection::ConnectionError),
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    CfR2SdkOperationError(#[from] cf_r2_sdk::error::OperationError),
}
