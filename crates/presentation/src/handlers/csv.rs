use crate::models::rwlock_shared_state::RwLockSharedState;
use application::usecase::csv::{
    depreiation::{DepreiationCsvJson, DepreiationCsvOutputs},
    item::{ItemCsvJson, ItemCsvOutputs},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use domain::value_object::error::{AppError, ResponseError};

#[utoipa::path(
    get,
    path = "/api/csv/depreiation",
    tag = "Csv",
    responses(
        (status = 200, description = "OK", body = DepreiationCsvJson),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn depreiation_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached csv/depreiation handler.");
    let shared_model = shared_state.read().await;
    // operation
    let outputs = DepreiationCsvOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().depreiation_csv,
    )
    .await;
    let result = outputs.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/csv/item",
    tag = "Csv",
    responses(
        (status = 200, description = "OK", body = ItemCsvJson),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn item_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached csv/item handler.");
    let shared_model = shared_state.read().await;
    // operation
    let outputs = ItemCsvOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().item_csv,
    )
    .await;
    let result = outputs.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}
