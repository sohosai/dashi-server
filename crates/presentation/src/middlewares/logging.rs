use axum::{extract::Request, middleware::Next, response::IntoResponse};
use domain::value_object::error::AppError;

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    //* preprocess *//
    tracing::info!("Preprocess");
    tracing::info!(
        "Method: {}, URI: {}, headers: {:?}, request: {:?}",
        request.method(),
        request.uri(),
        request.headers(),
        request.body()
    );
    //* handler *//
    tracing::info!("Handler");
    let response = next.run(request).await;
    //* postprocess *//
    tracing::info!("Success!");
    Ok(response)
}
