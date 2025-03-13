use super::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error(transparent)]
    DotEnvNotFountError(#[from] dotenvy::Error),
    #[error(transparent)]
    DotEnvVarError(#[from] std::env::VarError),
    #[error("Failed to get {0}")]
    DotEnvVarNotFountError(String),
    #[error(transparent)]
    ObjectStrageError(#[from] cf_r2_sdk::error::BuilderError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<ConnectionError> for AppError {
    fn from(error: ConnectionError) -> Self {
        match error {
            ConnectionError::DotEnvNotFountError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-not-found".to_string(),
                message: e.to_string(),
            },
            ConnectionError::DotEnvVarError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-var".to_string(),
                message: e.to_string(),
            },
            ConnectionError::DotEnvVarNotFountError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/dotenv-var-not-found".to_string(),
                message: format!("Failed to get {}", e),
            },
            ConnectionError::ObjectStrageError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/object-strage".to_string(),
                message: "ObjectStrageError: ObjectStrage trouble is occurred.".to_string(),
            },
            ConnectionError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
            ConnectionError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            ConnectionError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "connection/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
