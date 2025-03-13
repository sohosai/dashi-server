use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthCheckError {
    #[error("IncompatibleInItemTableError: Incompatible error is occurred in Item Table.")]
    IncompatibleInItemTableError,
    #[error("IncompatibleInLabelTableError: Incompatible error is occurred in Label Table.")]
    IncompatibleInLabelTableError,
    #[error("IncompatibleInMeiliSearchError: Incompatible error is occurred in MeiliSearch.")]
    IncompatibleInMeiliSearchError,
    #[error("IncompatibleInGraphDBError: Incompatible error is occurred in GraphDB.")]
    IncompatibleInGraphDBError,
    #[error("RootItemNotFoundError: Root item is not found.")]
    RootItemNotFoundError,
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    ConnectionError(#[from] crate::value_object::error::connection::ConnectionError),
}

impl From<HealthCheckError> for AppError {
    fn from(error: HealthCheckError) -> Self {
        match error {
            HealthCheckError::IncompatibleInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/incompatible-in-item-table".to_string(),
                message:
                    "IncompatibleInItemTableError: incompatible error is occurred in Item Table."
                        .to_string(),
            },
            HealthCheckError::IncompatibleInLabelTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/incompatible-in-label-table".to_string(),
                message:
                    "IncompatibleInLabelTableError: incompatible error is occurred in Label Table."
                        .to_string(),
            },
            HealthCheckError::IncompatibleInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/incompatible-in-meilisearch".to_string(),
                message:
                    "IncompatibleInMeiliSearchError: incompatible error is occurred in MeiliSearch."
                        .to_string(),
            },
            HealthCheckError::IncompatibleInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/incompatible-in-graphdb".to_string(),
                message: "IncompatibleInGraphDBError: incompatible error is occurred in GraphDB."
                    .to_string(),
            },
            HealthCheckError::RootItemNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/root-item-not-found".to_string(),
                message: "RootItemNotFoundError: Root item is not found.".to_string(),
            },
            HealthCheckError::GraphDBDeError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/graphdb-de".to_string(),
                message: "GraphDBDeError: Parse error of GraphDB object is occurred.".to_string(),
            },
            HealthCheckError::ConnectionError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/connection".to_string(),
                message: "ConnectionError: Connection trouble is occurred.".to_string(),
            },
            HealthCheckError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
            HealthCheckError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            HealthCheckError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "healthcheck/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
