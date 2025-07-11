use crate::value_object::error::AppError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisterItemError {
    //presentation
    #[error(transparent)]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("ParseIntError: Can not parse String to i32.")]
    ParseIntError,
    #[error("ParseBooleanError: Can not parse String to bool.")]
    ParseBooleanError,
    #[error("UnknownParameterError: Unknown parameter {0}.")]
    UnknownParameterError(String),
    #[error("ParameterNotFoundError: Parameter not found.")]
    ParameterNotFoundError,
    //infrastracture
    #[error(transparent)]
    DiscordWebHookError(#[from] crate::value_object::error::discord::sender::DiscordWebHookError),
    #[error(transparent)]
    ObjectStrageError(#[from] crate::value_object::error::object_strage::r2::R2Error),
    #[error("ItemNameEmptyError: Item name is empty.")]
    ItemNameEmptyError,
    #[error("LabelNotFoundError: Label not found.")]
    LabelNotFoundError,
    #[error("VisibleIdExisInItemTabletError: VisibleId already exists in Item Table.")]
    VisibleIdExistInItemTableError,
    #[error("VisibleIdConflictnItemTableError: Conflict VisibleId in Item Table.")]
    VisibleIdConflictInItemTableError,
    #[error("VisibleIdExistInMeiliSerachError: VisibleId already exists in MeiliSerach.")]
    VisibleIdExistInMeiliSerachError,
    #[error("VisibleIdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach.")]
    VisibleIdConflictInMeiliSerachError,
    #[error("ParentVisibleIdNotFoundInItemTableError: Parent VisibleId not found in Item Table.")]
    ParentVisibleIdNotFoundInItemTableError,
    #[error(
        "ParentVisibleIdNotFoundInMeiliSearchError: Parent VisibleId not found in MeiliSearch."
    )]
    ParentVisibleIdNotFoundInMeiliSearchError,
    #[error("VisibleIdConflictInGraphDBError: Conflict VisibleId in GraphDB.")]
    VisibleIdConflictInGraphDBError,
    #[error("VisibleIdNotFoundInGraphDBError: VisibleId not found in GraphDB.")]
    VisibleIdNotFoundInGraphDBError,
    #[error("RegisteredItemNotFoundError: Registered item not found.")]
    RegisteredItemNotFoundError,
    #[error(transparent)]
    GraphDBDeError(#[from] neo4rs::DeError),
    #[error(transparent)]
    GraphDBError(#[from] neo4rs::Error),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    RDBError(#[from] sea_orm::DbErr),
}

impl From<RegisterItemError> for AppError {
    fn from(error: RegisterItemError) -> Self {
        match error {
            RegisterItemError::MultipartError(_e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/multipart".to_string(),
                message: "MultipartError: Invaild form of mulitipart/data-form".to_string(),
            },
            RegisterItemError::ParseIntError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parse-int".to_string(),
                message: "ParseIntError: Can not parse String to i32.".to_string(),
            },
            RegisterItemError::ParseBooleanError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parse-boolean".to_string(),
                message: "ParseBooleanError: Can not parse String to bool.".to_string(),
            },
            RegisterItemError::UnknownParameterError(e) => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/unknown-parameter".to_string(),
                message: format!("UnknownParameterError: Unknown parameter {}.", e),
            },
            RegisterItemError::ParameterNotFoundError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parameter-not-found".to_string(),
                message: "ParameterNotFoundError: Parameter not found.".to_string(),
            },
            RegisterItemError::DiscordWebHookError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/discord-webhook".to_string(),
                message: format!("{}", e),
            },
            RegisterItemError::ObjectStrageError(e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/object-strage".to_string(),
                message: format!("ObjectStrageError: {}", e),
            },
            RegisterItemError::ItemNameEmptyError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/item-name-empty".to_string(),
                message: "ItemNameEmptyError: Item name is empty.".to_string(),
            },
            RegisterItemError::LabelNotFoundError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/label-not-found".to_string(),
                message: "LabelNotFoundError: Label not found.".to_string(),
            },
            RegisterItemError::VisibleIdExistInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-exist-in-item-table".to_string(),
                message: "VisibleIdExisInItemTabletError: VisibleId already exists in Item Table."
                    .to_string(),
            },
            RegisterItemError::VisibleIdConflictInItemTableError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/conflict-in-item-table".to_string(),
                message: "VisibleIdConflictnItemTableError: Conflict VisibleId in Item Table."
                    .to_string(),
            },
            RegisterItemError::VisibleIdExistInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-exist-in-meilisearch".to_string(),
                message:
                    "VisibleIdExistInMeiliSerachError: VisibleId already exists in MeiliSerach."
                        .to_string(),
            },
            RegisterItemError::VisibleIdConflictInMeiliSerachError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/conflict-in-meilisearch".to_string(),
                message: "VisibleIdConflictInMeiliSerachError: Conflict VisibleId in MeiliSerach."
                    .to_string(),
            },
            RegisterItemError::ParentVisibleIdNotFoundInItemTableError => AppError {
                status_code: StatusCode::BAD_REQUEST,
                code: "register-item/parent-visible-id-not-found".to_string(),
                message: "ParentVisibleIdNotFoundInItemTableError: Parent VisibleId not found in Item Table."
                    .to_string(),
            },
            RegisterItemError::ParentVisibleIdNotFoundInMeiliSearchError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/parent-visible-id-not-found".to_string(),
                message: "ParentVisibleIdNotFoundInMeiliSearchError: Parent VisibleId not found in MeiliSerch."
                    .to_string(),
            },
            RegisterItemError::VisibleIdConflictInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-conflict-in-graphdb".to_string(),
                message: "VisibleIdConflictInGraphDBError: Conflict VisibleId in GraphDB.".to_string(),
            },
            RegisterItemError::VisibleIdNotFoundInGraphDBError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/visible-id-not-found-in-graphdb".to_string(),
                message: "VisibleIdNotFoundInGraphDBError: VisibleId not found in GraphDB.".to_string(),
            },
            RegisterItemError::RegisteredItemNotFoundError => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/registered-item-not-found".to_string(),
                message: "RegisteredItemNotFoundError: Registered item not found.".to_string(),
            },
            RegisterItemError::GraphDBDeError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/graphdb-de".to_string(),
                message: "GraphDBDeError: GraphDB object can not be deserialize.".to_string(),
            },
            RegisterItemError::GraphDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/graphdb".to_string(),
                message: "GraphDBError: GraphDB trouble is occurred.".to_string(),
            },
            RegisterItemError::MeiliSearchError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/meilisearch".to_string(),
                message: "MeiliSearchError: MeiliSearchDB trouble is occurred.".to_string(),
            },
            RegisterItemError::RDBError(_e) => AppError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                code: "register-item/rdb".to_string(),
                message: "RDBError: RDB trouble is occurred.".to_string(),
            },
        }
    }
}
