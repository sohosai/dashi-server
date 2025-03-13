use crate::models::rwlock_shared_state::RwLockSharedState;
use application::usecase::rental::{
    rent::{RentRentalInputs, RentRentalOutputs},
    replace::{ReplaceRentalInputs, ReplaceRentalOutputs},
    update::{UpdateRentalInputs, UpdateRentalOutputs},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{entity::data_type::rental::RentalData, value_object::error::AppError};

#[utoipa::path(
    patch,
    path = "/api/rental/rent/{id}",
    tag = "Rental",
    params(("id", Path, description = "set item id (not visible id)")),
    request_body(
        description = "RentalData",
        content = RentalData,
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn rent_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    Json(rental_data): Json<RentalData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached rental/rent handler.");
    tracing::info!("path (id): {}", id);
    tracing::info!("body (rental_data): {:?}", rental_data);
    let shared_model = shared_state.write().await;
    // operation
    let inputs = RentRentalInputs { rental_data };
    let outputs = RentRentalOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().rent_rental,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/rental/update/{id}",
    tag = "Rental",
    params(("id", Path, description = "set item id (not visible id)")),
    request_body(
        description = "RentalData",
        content = RentalData,
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn update_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    Json(rental_data): Json<RentalData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached rental/update handler.");
    tracing::info!("path (id): {}", id);
    tracing::info!("body (rental_data): {:?}", rental_data);
    let shared_model = shared_state.write().await;
    // operation
    let inputs = UpdateRentalInputs { rental_data };
    let outputs = UpdateRentalOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().update_rental,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/rental/replace/{id}",
    tag = "Rental",
    params(("id", Path, description = "set item id (not visible id)")),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn replace_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached rental/replace handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.write().await;
    // operation
    let inputs = ReplaceRentalInputs { id };
    let outputs = ReplaceRentalOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().replace_rental,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}
