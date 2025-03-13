use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterConnectorError {
    #[error("RegisteredConnectorNotFoundError: Registered connector not found.")]
    RegisteredConnectorNotFoundError,
    #[error("ConflictConnectorNameError: Conflict Name in Connector Table.")]
    ConflictConnectorNameError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<RegisterConnectorError> for AppError {
    fn from(error: RegisterConnectorError) -> Self {
        match error {
            RegisterConnectorError::RegisteredConnectorNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-connector/registered-connector-not-found".to_string(),
                message: "RegisteredConnectorNotFoundError: Registered connector not found."
                    .to_string(),
            },
            RegisterConnectorError::ConflictConnectorNameError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-connector/conflict-connector-name".to_string(),
                message: "ConflictConnectorNameError: Conflict Name in Connector Table."
                    .to_string(),
            },
            RegisterConnectorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-connector/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            RegisterConnectorError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-connector/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
