use crate::models::rwlock_shared_state::RwLockSharedState;
use application::usecase::connector::{
    all_connectors::AllConnectorsOutputs,
    register::{RegisterConnectorInputs, RegisterConnectorOutputs},
    search::{SearchConnectorInputs, SearchConnectorOutputs},
    status::{StatusConnectorInputs, StatusConnectorOutputs},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    entity::data_type::{
        register_connector::RegisterConnectorData, status_connector::StatusConnectorData,
    },
    value_object::error::AppError,
};
use std::collections::HashMap;

#[utoipa::path(
    post,
    path = "/api/connector",
    tag = "Connector",
    request_body(
        description = "RegisterConnectorData",
        content = RegisterConnectorData,
    ),
    responses(
        (status = 201, description = "Created"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_connector_data): Json<RegisterConnectorData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached connector handler (POST).");
    tracing::info!(
        "body (register_connector_data): {:?}",
        register_connector_data
    );
    let shared_model = shared_state.write().await;
    //operation
    let inputs = RegisterConnectorInputs {
        register_connector_data,
    };
    let outputs = RegisterConnectorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().register_connector,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::CREATED, ()).into_response())
}

#[utoipa::path(
    get,
    path = "/api/connector",
    tag = "Connector",
    responses(
        (status = 200, description = "OK", body = AllConnectorsJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn all_connectors_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached connector handler (GET).");
    let shared_model = shared_state.write().await;
    //operation
    let outputs = AllConnectorsOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().all_connectors,
    )
    .await;
    let result = outputs.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/connector/search",
    tag = "Connector",
    params(("keywords", Query, description = "set search word")),
    responses(
        (status = 200, description = "OK", body = SearchConnectorJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn search_handler(
    Query(keywords): Query<HashMap<String, String>>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached connector/search handler.");
    tracing::info!("query (keywords): {:?}", keywords.get("keywords"));
    let shared_model = shared_state.write().await;
    let keywords = match keywords.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    //operation
    let inputs = SearchConnectorInputs {
        keywords: keywords.to_string(),
    };
    let outputs = SearchConnectorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().search_connector,
    )
    .await;
    let result = outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/connector/{id}",
    tag = "Connector",
    params(("id", Path, description = "set connector id (not name)")),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn status_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    Json(status_connector_data): Json<StatusConnectorData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached connector/status handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.write().await;
    tracing::info!("body (status_connector_data): {:?}", status_connector_data);
    //operation
    let inputs = StatusConnectorInputs {
        status_connector_data,
    };
    let outputs = StatusConnectorOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().status_connector,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}
