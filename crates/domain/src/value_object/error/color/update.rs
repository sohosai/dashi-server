use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateColorError {
    #[error("IdNotFoundInItemTableError: Id not found in Color Table.")]
    IdNotFoundInItemTableError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<UpdateColorError> for AppError {
    fn from(error: UpdateColorError) -> Self {
        match error {
            UpdateColorError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-color/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: Id not found in Color Table.".to_string(),
            },
            UpdateColorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-color/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            UpdateColorError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-color/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
