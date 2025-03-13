use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchColorError {
    #[error("EmptyKeywordsError: Keywords is empty.")]
    EmptyKeywordsError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}

impl From<SearchColorError> for AppError {
    fn from(error: SearchColorError) -> Self {
        match error {
            SearchColorError::EmptyKeywordsError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "search-color/empty-keywords".to_string(),
                message: "EmptyKeywordsError: Keywords is empty.".to_string(),
            },
            SearchColorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "search-color/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
        }
    }
}
