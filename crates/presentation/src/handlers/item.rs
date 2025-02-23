use application::usecase::item::{
    delete::{DeleteItemInputs, DeleteItemOutputs},
    individual::{IndividualItemInputs, IndividualItemOutputs},
    register::{RegisterItemInputs, RegisterItemOutputs},
    search::{SearchItemInputs, SearchItemJson, SearchItemOutputs},
    transfer::{TransferItemInputs, TransferItemOutputs},
    update::{UpdateItemDataJson, UpdateItemInputs, UpdateItemOutputs},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    entity::data_type::{register_item::RegisterItemData, transfer_item::TransferItemData},
    value_object::error::AppError,
};
use std::collections::HashMap;

use crate::models::rwlock_shared_state::RwLockSharedState;

#[utoipa::path(
    get,
    path = "/api/item/search",
    tag = "Item",
    params(("keywords", Query, description = "set search word")),
    responses(
        (status = 200, description = "OK", body = SearchItemJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn search_handler(
    Query(keywords): Query<HashMap<String, String>>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/search handler.");
    tracing::info!("query (keywords): {:?}", keywords.get("keywords"));
    let keywords = match keywords.get("keywords") {
        Some(keywords) => keywords,
        None => "",
    };
    let shared_model = shared_state.read().await;
    // operation
    let inputs = SearchItemInputs {
        keywords: keywords.to_string(),
    };
    let outputs = SearchItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().search_item,
    )
    .await;
    let result = outputs.run(inputs).await?;
    let result = SearchItemJson {
        search_items: result,
    };
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[utoipa::path(
    get,
    path = "/api/item/{id}",
    tag = "Item",
    params(("id", Path, description = "set item id (not visible id)")),
    responses(
        (status = 200, description = "OK", body = IndividualItemDataJson),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn individual_item_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/individual_item handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.read().await;
    //operation
    let inputs = IndividualItemInputs { id };
    let outputs = IndividualItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().individual_item,
    )
    .await;
    let result = outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn archive_handler(
    Path(limit): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> String {
    tracing::info!("reached item/archive handler.");
    tracing::info!("path (limit): {}", limit);
    let shared_model = shared_state.read().await;
    //operation
    drop(shared_model);
    "archive_handler".to_string()
}

#[utoipa::path(
    post,
    path = "/api/item/register",
    tag = "Item",
    request_body(
        description = "RegisterItemData",
        content = RegisterItemData,
    ),
    responses(
        (status = 201, description = "Created"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(register_item_data): Json<RegisterItemData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/register handler.");
    tracing::info!("body (register_item_data): {:?}", register_item_data);
    let shared_model = shared_state.write().await;
    // operation
    let inputs = RegisterItemInputs { register_item_data };
    let outputs = RegisterItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().register_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::CREATED, ()).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/item/update/{id}",
    tag = "Item",
    params(("id", Path, description = "set item id (not visible id)")),
    request_body(
        description = "UpdateItemDataJson",
        content = UpdateItemDataJson,
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
    Json(update_item_data_json): Json<UpdateItemDataJson>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/update handler.");
    tracing::info!("path (id): {}", id);
    tracing::info!("body (update_item_data_json): {:?}", update_item_data_json);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = UpdateItemInputs {
        id,
        update_item_data_json,
    };
    let outputs = UpdateItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().update_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    delete,
    path = "/api/item/delete/{id}",
    tag = "Item",
    params(("id", Path, description = "set item id (not visible id)")),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn delete_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/delete handler.");
    tracing::info!("path (id): {}", id);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = DeleteItemInputs { id };
    let outputs = DeleteItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().delete_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    patch,
    path = "/api/item/transfer",
    tag = "Item",
    request_body(
        description = "TransferItemData",
        content = TransferItemData,
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
)]
pub async fn transfer_handler(
    State(shared_state): State<RwLockSharedState>,
    Json(transfer_item_data): Json<TransferItemData>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/transfer handler.");
    tracing::info!("body (transfer_item_data): {:?}", transfer_item_data);
    let shared_model = shared_state.write().await;
    //operation
    let inputs = TransferItemInputs { transfer_item_data };
    let outputs = TransferItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().transfer_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}
