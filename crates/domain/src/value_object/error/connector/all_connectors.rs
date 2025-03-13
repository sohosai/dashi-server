use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AllConnectorsError {
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<AllConnectorsError> for AppError {
    fn from(error: AllConnectorsError) -> Self {
        match error {
            AllConnectorsError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "all-connectors/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
