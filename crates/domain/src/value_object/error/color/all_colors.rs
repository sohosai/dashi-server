use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AllColorsError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<AllColorsError> for AppError {
    fn from(error: AllColorsError) -> Self {
        match error {
            AllColorsError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "all-colors/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
