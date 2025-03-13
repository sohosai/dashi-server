use thiserror::Error;

#[derive(Debug, Error)]
pub enum InsertCsvDataError {
    #[error("InvalidConnectorNameError: Invalid connector name.")]
    InvalidConnectorNameError,
    #[error("InvalidColorHexColorCodeError: Invalid color hex color code.")]
    InvalidColorHexColorCodeError,
    #[error("InvalidColorNameError: Invalid color name.")]
    InvalidColorNameError,
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
    #[error(transparent)]
    MeiliSearchError(#[from] meilisearch_sdk::errors::Error),
}
