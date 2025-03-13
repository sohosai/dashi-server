use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrashItemError {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<TrashItemError> for AppError {
    fn from(error: TrashItemError) -> Self {
        match error {
            TrashItemError::SerdeJsonError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "trash-item/serde-json".to_string(),
                message: "SerdeJsonError: translate Value to Vec<String> was failed.".to_string(),
            },
            TrashItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "trash-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
