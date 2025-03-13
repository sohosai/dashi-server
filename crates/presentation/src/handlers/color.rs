use crate::models::rwlock_shared_state::RwLockSharedState;
use application::usecase::color::{
    all_colors::AllColorsOutputs,
    register::{RegisterColorInputs, RegisterColorOutputs},
    search::{SearchColorInputs, SearchColorOutputs},
    update::{UpdateColorInputs, UpdateColorOutputs},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    entity::data_type::{register_color::RegisterColorData, update_color::UpdateColorData},
    value_object::error::AppError,
};
use std::collections::HashMap;

#[utoipa::path(
    post,
    path = "/api/color",
    tag = "Color",
    request_body(
        description = "RegisterColorData",
        content = RegisterColorData,
    ),
    responses(
        (status = 201, description = "Created"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_color_data): Json<RegisterColorData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached color handler (POST).");
    tracing::info!("body (register_color_data): {:?}", register_color_data);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = RegisterColorInputs {
        register_color_data,
    };
    let outputs = RegisterColorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().register_color,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::CREATED, ()).into_response())
}

#[utoipa::path(
    get,
    path = "/api/color",
    tag = "Color",
    responses(
        (status = 200, description = "OK", body = AllColorsJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn all_colors_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached color handler (GET).");
    let shared_model = shared_state.write().await;
    //operation
    let outputs = AllColorsOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().all_colors,
    )
    .await;
    let result = outputs.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/color/search",
    tag = "Color",
    params(("keywords", Query, description = "set search word")),
    responses(
        (status = 200, description = "OK", body = SearchColorJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn search_handler(
    Query(keywords): Query<HashMap<String, String>>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached color/search handler.");
    tracing::info!("query (keywords): {:?}", keywords.get("keywords"));
    let shared_model = shared_state.write().await;
    let keywords = match keywords.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    //operation
    let inputs = SearchColorInputs {
        keywords: keywords.to_string(),
    };
    let outputs = SearchColorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().search_color,
    )
    .await;
    let result = outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/color/{id}",
    tag = "Color",
    params(("id", Path, description = "set color id (not name)")),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn update_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    Json(update_color_data): Json<UpdateColorData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached color/update handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.write().await;
    tracing::info!("body (update_color_data): {:?}", update_color_data);
    //operation
    let inputs = UpdateColorInputs { update_color_data };
    let outputs = UpdateColorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().update_color,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}
