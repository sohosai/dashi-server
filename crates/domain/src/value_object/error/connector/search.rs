use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchConnectorError {
    #[error("EmptyKeywordsError: Keywords is empty.")]
    EmptyKeywordsError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}

impl From<SearchConnectorError> for AppError {
    fn from(error: SearchConnectorError) -> Self {
        match error {
            SearchConnectorError::EmptyKeywordsError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "search-connector/empty-keywords".to_string(),
                message: "EmptyKeywordsError: Keywords is empty.".to_string(),
            },
            SearchConnectorError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "search-connector/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
        }
    }
}
