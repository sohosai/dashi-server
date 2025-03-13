use thiserror::Error;

#[derive(Debug, Error)]
pub enum R2Error {
    #[error(transparent)]
    ConnectionError(#[from] crate::value_object::error::connection::ConnectionError),
    #[error(transparent)]
    WebpError(#[from] crate::value_object::error::object_strage::webp::WebpError),
    #[error(transparent)]
    CfR2SdkOperationError(#[from] cf_r2_sdk::error::OperationError),
}
