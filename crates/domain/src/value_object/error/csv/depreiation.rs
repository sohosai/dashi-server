use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DepreiationCsvError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<DepreiationCsvError> for AppError {
    fn from(error: DepreiationCsvError) -> Self {
        match error {
            DepreiationCsvError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "depreiation-csv/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
