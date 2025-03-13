use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndividualItemError {
    #[error("LabelNotFoundError: Label not found.")]
    LabelNotFoundError,
    #[error("ParentLabelNotFoundError: Parent Label not found.")]
    ParentLabelNotFoundError,
    #[error("IdNotFoundInItemTableError: Id not found in Item Table.")]
    IdNotFoundInItemTableError,
    #[error("IdConflictInItemTableError: Conflict Id in Item Table.")]
    IdConflictInItemTableError,
    #[error("IdNotFoundInGraphDBError: Id not found in Graph.")]
    IdNotFoundInGraphDBError,
    #[error("ParentIdConflictInGraphError: Conflict Id in Graph.")]
    ParentIdConflictInItemTableError,
    #[error("ParentIdNotFoundInItemTableError: Parent Id not found in Item Table.")]
    ParentIdNotFoundInItemTableError,
    #[error("IdConflictInGraphError: Conflict Id in Graph.")]
    IdConflictInGraphError,
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
}

impl From<IndividualItemError> for AppError {
    fn from(error: IndividualItemError) -> Self {
        match error {
            IndividualItemError::ParentLabelNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/label-not-found".to_string(),
                message: "LabelNotFoundError: Label not found.".to_string(),
            },
            IndividualItemError::LabelNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/label-not-found".to_string(),
                message: "LabelNotFoundError: Label not found.".to_string(),
            },
            IndividualItemError::IdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "individual-item/id-not-found-in-item-table".to_string(),
                message: "IdNotFoundInItemTableError: Id not found in Item Table.".to_string(),
            },
            IndividualItemError::IdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/id-conflict-in-item-table".to_string(),
                message: "IdConflictInItemTableError: Conflict Id in Item Table.".to_string(),
            },
            IndividualItemError::IdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "individual-item/id-not-found-in-graphdb".to_string(),
                message: "IdNotFoundInGraphDBError: Id not found in Graph.".to_string(),
            },
            IndividualItemError::ParentIdNotFoundInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/parent-id-not-found-in-item-table".to_string(),
                message: "ParentIdNotFoundInItemTableError: Parent Id not found in Item Table."
                    .to_string(),
            },
            IndividualItemError::ParentIdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/parent-id-conflict-in-item-table".to_string(),
                message: "ParentIdConflictInItemTableError: Conflict Id in Item Table.".to_string(),
            },
            IndividualItemError::IdConflictInGraphError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/id-conflict-in-graph".to_string(),
                message: "IdConflictInGraphError: Conflict Id in Graph.".to_string(),
            },
            IndividualItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "inidividual-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
            IndividualItemError::GraphDBDeError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "inidividual-item/graphdb-de".to_string(),
                message: "GraphDBDeError: Parse error of GraphDB object is occurred.".to_string(),
            },
            IndividualItemError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "individual-item/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
        }
    }
}
