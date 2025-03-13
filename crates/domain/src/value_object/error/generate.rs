use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error("IsMaxBreakError: IsMax=true * {0}")]
    IsMaxBreakError(String),
    #[error("UnderflowError: {0}")]
    UnderflowError(String),
    #[error("OverflowError: VisibleId has exceeded the issue limit. The current number of remaining VisibleId available for issuance is {0}.")]
    OverflowError(String),
    #[error("LabelModelNotFoundError: Cannot find the Label model (IsMax: true).")]
    LabelModelNotFoundError,
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<GenerateError> for AppError {
    fn from(error: GenerateError) -> Self {
        match error {
            GenerateError::IsMaxBreakError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/is-max-break".to_string(),
                message: format!("IsMaxBreakError: IsMax=true * {}", e),
            },
            GenerateError::UnderflowError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/underflow".to_string(),
                message: format!("UnderflowError: {}", e),
            },
            GenerateError::OverflowError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/overflow".to_string(),
                message: format!("OverflowError: VisibleId has exceeded the issue limit. The current number of remaining VisibleId available for issuance is {}.", e),
            },
            GenerateError::LabelModelNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/label-module-not-found".to_string(),
                message: "sLabelModelNotFoundError: Cannot find the Label model (IsMax: true).".to_string(),
            },
            GenerateError::ParseIntError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/parse-int".to_string(),
                message: e.to_string(),
            },
            GenerateError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "generate/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
