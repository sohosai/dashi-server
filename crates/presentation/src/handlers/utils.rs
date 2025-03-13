use application::usecase::{
    generate::{GenerateInputs, GenerateOutputs},
    utils::healthcheck::HealthCheckUseCase,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use domain::{
    entity::data_type::generate_data_request::GenerateDataRequest, value_object::error::AppError,
};

use crate::models::rwlock_shared_state::RwLockSharedState;

pub async fn login_handler(State(shared_state): State<RwLockSharedState>) -> String {
    tracing::info!("reached login handler.");
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "login_handler".to_string()
}

#[utoipa::path(
    get,
    path = "/api/healthcheck",
    tag = "Health Check",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn healthcheck_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached healthcheck handler.");
    let shared_model = shared_state.read().await;
    // operation
    let healthcheck_usecase = HealthCheckUseCase::new(shared_model.clone().healthcheck).await;
    healthcheck_usecase.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    post,
    path = "/api/generate",
    tag = "Generate",
    request_body(
        description = "GenerateDataRequest",
        content = GenerateDataRequest,
    ),
    responses(
        (status = 201, description = "Created", body = GenerateData),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn generate_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(generate_data): Json<GenerateDataRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached generate/nothing handler.");
    tracing::info!("path (generate_data): {:?}", generate_data);
    let shared_model = shared_state.write().await;
    // operation
    let generate_inputs = GenerateInputs { generate_data };
    let generate_outputs = GenerateOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().generate,
    )
    .await;
    let result = generate_outputs.run(generate_inputs).await?;
    drop(shared_model);
    Ok((StatusCode::CREATED, Json(result)).into_response())
}
