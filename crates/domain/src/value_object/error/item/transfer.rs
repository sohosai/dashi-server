use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransferItemError {
    #[error("CannotTransferRootItemError: Can not transfer root item.")]
    CannotTransferRootItemError,
    #[error("IdConflictInGraphDBError: Conflict Id in GraphDB.")]
    IdConflictInGraphDBError,
    #[error("IdNotFoundInGraphDBError: Id not found in GraphDb.")]
    IdNotFoundInGraphDBError,
    #[error("NewParentIdConflictInGraphDBError: Conflict NewParentId in GraphDB.")]
    NewParentIdConflictInGraphDBError,
    #[error("NewParentIdNotFoundInGraphDBError: NewParentId not found in GraphDB.")]
    NewParentIdNotFoundInGraphDBError,
    #[error("NewParentIdOneOfDescendantIdsError: NewParentId is one of descendant ids.")]
    NewParentIdOneOfDescendantIdsError,
    #[error("OldParentIdConflictInGraphDBError: Conflict OldParentId in GraphDB.")]
    OldParentIdConflictInGraphDBError,
    #[error("OldParentdNotFoundInGraphDBError: OldParentId not found in GraphDB.")]
    OldParentdNotFoundInGraphDBError,
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
}

impl From<TransferItemError> for AppError {
    fn from(error: TransferItemError) -> Self {
        match error {
            TransferItemError::CannotTransferRootItemError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/cannot-transfer-root-item".to_string(),
                message: "CannotTransferRootItemError: Can not transfer root item.".to_string(),
            },
            TransferItemError::IdConflictInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/id-conflict".to_string(),
                message: "Conflict Id in GraphDB.".to_string(),
            },
            TransferItemError::IdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "transfer-item/id-not-found".to_string(),
                message: "IdNotFoundInGraphDBError: Id not found in GraphDB.".to_string(),
            },
            TransferItemError::NewParentIdConflictInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/new-parent-id-conflict".to_string(),
                message: "NewParentIdConflictInGraphDBError: Conflict NewParentId in GraphDB."
                    .to_string(),
            },
            TransferItemError::NewParentIdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "transfer-item/new-parent-id-not-found".to_string(),
                message: "NewParentIdNotFoundInGraphDBError: NewParentId not found in GraphDB."
                    .to_string(),
            },
            TransferItemError::NewParentIdOneOfDescendantIdsError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/new-parent-id-one-of-descendant-ids".to_string(),
                message:
                    "NewParentIdOneOfDescendantIdsError: NewParentId is one of descendant ids."
                        .to_string(),
            },
            TransferItemError::OldParentIdConflictInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/old-parent-id-conflict".to_string(),
                message: "OldParentIdConflictInGraphDBError: Conflict OldParentId in GraphDB."
                    .to_string(),
            },
            TransferItemError::OldParentdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/old-parent-id-not-found".to_string(),
                message: "OldParentdNotFoundInGraphDBError: OldParentId not found in GraphDB."
                    .to_string(),
            },
            TransferItemError::GraphDBDeError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/graphdb-de".to_string(),
                message: "GraphDBDeError: GraphDB object can not be deserialize.".to_string(),
            },
            TransferItemError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "transfer-item/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
        }
    }
}
