use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitHealthCheckError {
    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error(transparent)]
    MigrationError(#[from] crate::error::migration::MigrationError),
    #[error(transparent)]
    InitError(#[from] crate::error::init::InitError),
    #[error(transparent)]
    HealthCheckError(#[from] domain::value_object::error::healthcheck::HealthCheckError),
}
