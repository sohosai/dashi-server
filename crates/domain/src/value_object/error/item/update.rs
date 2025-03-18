use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpdateItemError {
    #[error(transparent)]
    DiscordWebHookError(#[from] crate::value_object::error::discord::sender::DiscordWebHookError),
    #[error(transparent)]
    ObjectStrageError(#[from] crate::value_object::error::object_strage::r2::R2Error),
    #[error("CannotupdateRootItemError: Can not update root item.")]
    CannotupdateRootItemError,
    #[error("IdConflictInItemTableError: Conflict VisibleId in Item Table.")]
    IdConflictInItemTableError,
    #[error("IdNotFoundInItemTableError: VisibleId not found in Item Table.")]
    IdNotFoundInItemTableError,
    #[error("IdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach.")]
    IdConflictInMeiliSerachError,
    #[error("IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch.")]
    IdNotFoundInMeiliSearchError,
    #[error("ItemNameEmptyError: Item name is empty.")]
    ItemNameEmptyError,
    #[error("LabelNotFoundError: Label not found.")]
    LabelNotFoundError,
    #[error("ColorPatternExistInItemTableError: Color already exists in Item Table.")]
    ColorPatternExistInItemTableError,
    #[error("ColorPatternConflictInItemTableError: Conflict Color in Item Table.")]
    ColorPatternConflictInItemTableError,
    #[error("ColorPatternExistInMeiliSearchError: Color already exists in MeiliSearch.")]
    ColorPatternExistInMeiliSearcheError,
    #[error("ColorPatternConflictInMeiliSearchError: Conflict Color in Item MeiliSearch.")]
    ColorPatternConflictInMeiliSearchError,
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<UpdateItemError> for AppError {
    fn from(error: UpdateItemError) -> Self {
        match error {
            UpdateItemError::DiscordWebHookError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/discord-webhook".to_string(),
                message: format!("{}", e),
            },
            UpdateItemError::ObjectStrageError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/object-strage".to_string(),
                message: format!("ObjectStrageError: {}", e),
            },
            UpdateItemError::CannotupdateRootItemError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/cannot-update-root-item".to_string(),
                message: "CannotupdateRootItemError: Can not update root item.".to_string(),
            },
            UpdateItemError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            UpdateItemError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            UpdateItemError::IdConflictInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            UpdateItemError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            UpdateItemError::ItemNameEmptyError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/item-name-empty".to_string(),
                message: "Item name is empty.".to_string(),
            },
            UpdateItemError::LabelNotFoundError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/label-not-found".to_string(),
                message: "LabelNotFoundError: Label not found.".to_string(),
            },
            UpdateItemError::ColorPatternExistInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/color-pattern-exist-in-item-table".to_string(),
                message: "Color already exists in Item Table.".to_string(),
            },
            UpdateItemError::ColorPatternConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/color-pattern-conflict-in-item-table".to_string(),
                message: "Conflict Color in Item Table.".to_string(),
            },
            UpdateItemError::ColorPatternExistInMeiliSearcheError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "update-item/color-pattern-exist-in-meilisearch".to_string(),
                message: "Color already exists in MeiliSearch.".to_string(),
            },
            UpdateItemError::ColorPatternConflictInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/color-pattern-conflict-in-meilisearch".to_string(),
                message: "Conflict Color in Item MeiliSearch.".to_string(),
            },
            UpdateItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            UpdateItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "update-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
