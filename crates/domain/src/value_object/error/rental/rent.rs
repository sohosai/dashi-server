use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RentRentalError {
    #[error("RecipientEmptyError: Recipient is empty.")]
    RecipientEmptyError,
    #[error("IdConflictInItemTableError: Conflict VisibleId in Item Table.")]
    IdConflictInItemTableError,
    #[error("IdNotFoundInItemTableError: VisibleId not found in Item Table.")]
    IdNotFoundInItemTableError,
    #[error("IdConflictInMeiliSearchError: Conflict VisibleId in MeiliSerach.")]
    IdConflictInMeiliSearchError,
    #[error("IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch.")]
    IdNotFoundInMeiliSearchError,
    #[error("AlreadyRentedInItemTableError: Already rented in Item Table.")]
    AlreadyRentedInItemTableError,
    #[error("AlreadyRentedInMeiliSearchError: Already rented in MeiliSearch.")]
    AlreadyRentedInMeiliSearchError,
    #[error("DateTimeParseError: Parse DateTime is failed.")]
    ParseDateTimeError(String),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<RentRentalError> for AppError {
    fn from(error: RentRentalError) -> Self {
        match error {
            RentRentalError::RecipientEmptyError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/recipient-empty".to_string(),
                message: "RecipientEmptyError: Recipient is empty.".to_string(),
            },
            RentRentalError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "rent-rental/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            RentRentalError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            RentRentalError::IdConflictInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "rent-rental/id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSearchError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            RentRentalError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            RentRentalError::AlreadyRentedInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/already-rented-in-item-table".to_string(),
                message: "AlreadyRentedInItemTableError: Already rented in Item Table.".to_string(),
            },
            RentRentalError::AlreadyRentedInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/already-rented-in-meilisearch".to_string(),
                message: "AlreadyRentedInMeiliSearchError: Already rented in MeiliSearch."
                    .to_string(),
            },
            RentRentalError::ParseDateTimeError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "rent-rental/parse-datetime".to_string(),
                message: "ParseDateTimeError: Parse DateTime is failed.".to_string(),
            },
            RentRentalError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "rent-rental/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            RentRentalError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "rent-rental/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
