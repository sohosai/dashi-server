use axum::http::StatusCode;
use thiserror::Error;

use crate::value_object::error::AppError;

#[derive(Debug, Error)]
pub enum DeleteItemError {
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
    #[error("VisibleIdConflictInGraphDBError: Conflict VisibleId in GraphDB.")]
    VisibleIdConflictInGraphDBError,
    #[error("VisibleIdNotFoundInGraphDBError: VisibleId not found in GraphDB.")]
    VisibleIdNotFoundInGraphDBError,
    #[error("CannotDeleteRootItemError: Can not delete root item.")]
    CannotDeleteRootItemError,
    #[error("MultipleParentItemsError: Multiple parent items are exist.")]
    MultipleParentItemsError,
    #[error("ParentItemMissingError: Parent item is missing.")]
    ParentItemMissingError,
    #[error("ItemOnLoanError: Item is on loan.")]
    ItemOnLoanError,
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<DeleteItemError> for AppError {
    fn from(error: DeleteItemError) -> Self {
        match error {
            DeleteItemError::IsRentIsTrueError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/is-rent-is-true".to_string(),
                message: "IsRentIsTrueError: is_rent field is true.".to_string(),
            },
            DeleteItemError::DiscordWebHookError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/discord-webhook".to_string(),
                message: format!("{}", e),
            },
            DeleteItemError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/visible-id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            DeleteItemError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/visible-id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: VisibleId not found in Item Table."
                    .to_string(),
            },
            DeleteItemError::IdConflictInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/visible-id-conflict-in-meilisearch".to_string(),
                message: "IdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            DeleteItemError::IdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/visible-id-not-found-in-meilisearch".to_string(),
                message: "IdNotFoundInMeiliSearchError: VisibleId not found in MeiliSearch."
                    .to_string(),
            },
            DeleteItemError::VisibleIdConflictInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/visible-id-conflict-in-graphdb".to_string(),
                message: "VisibleIdConflictInGraphDBError: Conflict VisibleId in GraphDB."
                    .to_string(),
            },
            DeleteItemError::VisibleIdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/visible-id-not-found-in-graphdb".to_string(),
                message: "VisibleIdNotFoundInGraphDBError: VisibleId not found in GraphDB."
                    .to_string(),
            },
            DeleteItemError::CannotDeleteRootItemError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/cannot-delete-root-item".to_string(),
                message: "CannotDeleteRootItemError: Can not delete root item.".to_string(),
            },
            DeleteItemError::MultipleParentItemsError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/multiple-parent-items".to_string(),
                message: "MultipleParentItemsError: Multiple parent items are exist.".to_string(),
            },
            DeleteItemError::ParentItemMissingError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/parent-item-missing".to_string(),
                message: "ParentItemMissingError: Parent item is missing.".to_string(),
            },
            DeleteItemError::ItemOnLoanError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "delete-item/item-on-loan".to_string(),
                message: "ItemOnLoanError: Item is on loan.".to_string(),
            },
            DeleteItemError::GraphDBDeError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/graphdb".to_string(),
                message: "GraphDBDeError: GraphDB object can not be deserialize.".to_string(),
            },
            DeleteItemError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
            DeleteItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            DeleteItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "delete-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
