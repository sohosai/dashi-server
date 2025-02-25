use thiserror::Error;

#[derive(Debug, Error)]
pub enum PingError {
    #[error(transparent)]
    ConnectionError(#[from] domain::value_object::error::connection::ConnectionError),
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}
