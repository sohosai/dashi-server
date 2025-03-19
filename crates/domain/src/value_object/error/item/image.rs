use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageItemError {
    //presentation
    #[error(transparent)]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("UnknownParameterError: Unknown parameter {0}.")]
    UnknownParameterError(String),
    #[error("ParameterNotFoundError: Parameter not found.")]
    ParameterNotFoundError,
    //infrastracture
    #[error("IsRentIsTrueError: is_rent field is true.")]
    IsRentIsTrueError,
    #[error(transparent)]
    DiscordWebHookError(#[from] crate::value_object::error::discord::sender::DiscordWebHookError),
    #[error("IdConflictInItemTableError: Conflict VisibleId in Item Table.")]
    IdConflictInItemTableError,
    #[error("IdNotFoundInItemTableError: VisibleId not found in Item Table.")]
    IdNotFoundInItemTableError,
    #[error("IdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach.")]
    IdConflictInMeiliSerachError,
    #[error("IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch.")]
    IdNotFoundInMeiliSearchError,
    #[error(transparent)]
    ObjectStrageError(#[from] crate::value_object::error::object_strage::r2::R2Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<ImageItemError> for AppError {
    fn from(error: ImageItemError) -> Self {
        match error {
            ImageItemError::MultipartError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/multipart".to_string(),
                message: "MultipartError: Invaild form of mulitipart/data-form".to_string(),
            },
            ImageItemError::UnknownParameterError(e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/unknown-parameter".to_string(),
                message: format!("UnknownParameterError: Unknown parameter {}.", e),
            },
            ImageItemError::ParameterNotFoundError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/parameter-not-found".to_string(),
                message: "ParameterNotFoundError: Parameter not found.".to_string(),
            },
            ImageItemError::IsRentIsTrueError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/is-rent-is-true".to_string(),
                message: "IsRentIsTrueError: is_rent field is true.".to_string(),
            },
            ImageItemError::DiscordWebHookError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/discord-webhook".to_string(),
                message: format!("{}", e),
            },
            ImageItemError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            ImageItemError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            ImageItemError::IdConflictInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            ImageItemError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "image-item/id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            ImageItemError::ObjectStrageError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/object-strage".to_string(),
                message: format!("ObjectStrageError: {}", e),
            },
            ImageItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            ImageItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "image-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
