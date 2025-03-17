use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AllRentalItemsError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<AllRentalItemsError> for AppError {
    fn from(error: AllRentalItemsError) -> Self {
        match error {
            AllRentalItemsError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "all-rental_items/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
