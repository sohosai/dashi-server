use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("{0}")]
    RunShellScriptError(String),
}
