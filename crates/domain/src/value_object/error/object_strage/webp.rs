use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebpError {
    #[error("OutOfRangeError: Quality must be between 0 and 100")]
    OutOfRangeError,
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("{0}")]
    EncoderError(String),
}
