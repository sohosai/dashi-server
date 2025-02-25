use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("{0}")]
    RunShellScriptError(String),
    #[error(transparent)]
    DotenvyError(#[from] dotenvy::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error("VarGetError: Failed to get {0}.")]
    VarGetError(String),
}
