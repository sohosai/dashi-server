use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiscordWebHookError {
    #[error(transparent)]
    ConnectionError(#[from] crate::value_object::error::connection::ConnectionError),
    #[error("DateTimeParseError: Parse DateTime is failed.")]
    ParseDateTimeError(String),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl From<DiscordWebHookError> for AppError {
    fn from(error: DiscordWebHookError) -> Self {
        match error {
            DiscordWebHookError::ConnectionError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "discord-webhook/connection".to_string(),
                message: format!("{}", e),
            },
            DiscordWebHookError::ParseDateTimeError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "discord-webhook/parse-datetime".to_string(),
                message: "ParseDateTimeError: Parse DateTime is failed.".to_string(),
            },
            DiscordWebHookError::SerdeJsonError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "discord-webhook/serde-json".to_string(),
                message: "SerdeJsonError: Parse discord webhook json trouble is occurred."
                    .to_string(),
            },
            DiscordWebHookError::ReqwestError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "discord-webhook/reqwest".to_string(),
                message: "ReqwestError: Send discord webhook trouble is occurred.".to_string(),
            },
        }
    }
}
