use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemCsvError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<ItemCsvError> for AppError {
    fn from(error: ItemCsvError) -> Self {
        match error {
            ItemCsvError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "item-csv/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
