use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterColorError {
    #[error("RegisteredColorNotFoundError: Registered color not found.")]
    RegisteredColorNotFoundError,
    #[error("ConflictColorNameError: Conflict Name in Color Table.")]
    ConflictColorNameError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<RegisterColorError> for AppError {
    fn from(error: RegisterColorError) -> Self {
        match error {
            RegisterColorError::RegisteredColorNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-color/registered-color-not-found".to_string(),
                message: "RegisteredColorNotFoundError: Registered color not found.".to_string(),
            },
            RegisterColorError::ConflictColorNameError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-color/conflict-color-name".to_string(),
                message: "ConflictColorNameError: Conflict Name in Color Table.".to_string(),
            },
            RegisterColorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-color/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            RegisterColorError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-color/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
