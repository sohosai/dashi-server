use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::value_object::error::{AppError, ResponseError};

#[utoipa::path(
    get,
    path = "/api/joke/unavailable",
    tag = "Joke",
    responses(
        (status = 451, description = "Unavailable For Legal Reasons", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn unavailable_handler() -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached joke/unavailable handler.");
    Ok((
        StatusCode::IM_A_TEAPOT,
        Json(ResponseError {
            code: "joke/unavailable".to_string(),
            message: "Unavailable for sopotan reasons".to_string(),
        }),
    )
        .into_response())
}

#[utoipa::path(
    get,
    path = "/api/joke/teapot",
    tag = "Joke",
    responses(
        (status = 418, description = "I'm A Teapot", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn teapot_handler() -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached joke/teapot handler.");
    Ok((
        StatusCode::IM_A_TEAPOT,
        Json(ResponseError {
            code: "joke/teapot".to_string(),
            message: "I'm a sopotan!".to_string(),
        }),
    )
        .into_response())
}
