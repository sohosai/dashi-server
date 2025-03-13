use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitHealthCheckError {
    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error(transparent)]
    MigrationError(#[from] crate::error::migration::MigrationError),
    #[error(transparent)]
    InitializeError(#[from] crate::error::initialize::InitializeError),
}
