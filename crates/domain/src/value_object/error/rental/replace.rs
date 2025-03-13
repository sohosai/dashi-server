use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplaceRentalError {
    #[error("IdConflictInItemTableError: Conflict VisibleId in Item Table.")]
    IdConflictInItemTableError,
    #[error("IdNotFoundInItemTableError: VisibleId not found in Item Table.")]
    IdNotFoundInItemTableError,
    #[error("IdConflictInMeiliSearchError: Conflict VisibleId in MeiliSerach.")]
    IdConflictInMeiliSearchError,
    #[error("IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch.")]
    IdNotFoundInMeiliSearchError,
    #[error("ItemNotRentedInItemTableError: Item not rented in Item Table.")]
    ItemNotRentedInItemTableError,
    #[error("ItemNotRentedInMeiliSearchError: Item not rented in MeiliSearch.")]
    ItemNotRentedInMeiliSearchError,
    #[error("DateTimeParseError: Parse DateTime is failed.")]
    ParseDateTimeError(String),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<ReplaceRentalError> for AppError {
    fn from(error: ReplaceRentalError) -> Self {
        match error {
            ReplaceRentalError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "replace-rental/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            ReplaceRentalError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "replace-rental/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            ReplaceRentalError::IdConflictInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "replace-rental/id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSearchError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            ReplaceRentalError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "replace-rental/id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            ReplaceRentalError::ItemNotRentedInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "replace-rental/item-not-rented-in-item-table".to_string(),
                message: "ItemNotRentedInItemTableError: Item not rented in Item Table."
                    .to_string(),
            },
            ReplaceRentalError::ItemNotRentedInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "replace-rental/item-not-rented-in-meilisearch".to_string(),
                message: "ItemNotRentedInMeiliSearchError: Item not rented in MeiliSearch."
                    .to_string(),
            },
            ReplaceRentalError::ParseDateTimeError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "replace-rental/parse-datetime".to_string(),
                message: "ParseDateTimeError: Parse DateTime is failed.".to_string(),
            },
            ReplaceRentalError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "replace-rental/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            ReplaceRentalError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "replace-rental/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
