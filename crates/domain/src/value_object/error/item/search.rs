use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchItemError {
    #[error("EmptyKeywordsError: Keywords is empty.")]
    EmptyKeywordsError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}

impl From<SearchItemError> for AppError {
    fn from(error: SearchItemError) -> Self {
        match error {
            SearchItemError::EmptyKeywordsError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "search-item/empty-keywords".to_string(),
                message: "EmptyKeywordsError: Keywords is empty.".to_string(),
            },
            SearchItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "search-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
        }
    }
}
