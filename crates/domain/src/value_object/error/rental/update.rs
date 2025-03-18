use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateRentalError {
    #[error(transparent)]
    DiscordWebHookError(#[from] crate::value_object::error::discord::sender::DiscordWebHookError),
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

impl From<UpdateRentalError> for AppError {
    fn from(error: UpdateRentalError) -> Self {
        match error {
            UpdateRentalError::DiscordWebHookError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "rent-rental/discord-webhook".to_string(),
                message: format!("{}", e),
            },
            UpdateRentalError::RecipientEmptyError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/recipient-empty".to_string(),
                message: "RecipientEmptyError: Recipient is empty.".to_string(),
            },
            UpdateRentalError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-rental/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            UpdateRentalError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            UpdateRentalError::IdConflictInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-rental/id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSearchError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            UpdateRentalError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            UpdateRentalError::ItemNotRentedInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/item-not-rented-in-item-table".to_string(),
                message: "ItemNotRentedInItemTableError: Item not rented in Item Table."
                    .to_string(),
            },
            UpdateRentalError::ItemNotRentedInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/item-not-rented-in-meilisearch".to_string(),
                message: "ItemNotRentedInMeiliSearchError: Item not rented in MeiliSearch."
                    .to_string(),
            },
            UpdateRentalError::ParseDateTimeError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-rental/parse-datetime".to_string(),
                message: "ParseDateTimeError: Parse DateTime is failed.".to_string(),
            },
            UpdateRentalError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-rental/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            UpdateRentalError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-rental/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
