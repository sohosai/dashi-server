use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatusConnectorError {
    #[error("IdNotFoundInItemTableError: Id not found in Connector Table.")]
    IdNotFoundInItemTableError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<StatusConnectorError> for AppError {
    fn from(error: StatusConnectorError) -> Self {
        match error {
            StatusConnectorError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "status-connector/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: Id not found in Connector Table.".to_string(),
            },
            StatusConnectorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "status-connector/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            StatusConnectorError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "status-connector/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
