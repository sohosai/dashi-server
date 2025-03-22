use crate::{
    models::{
        image_item_multipart::ImageItemMultipartData,
        register_item_multipart_data::RegisterItemMultipartData,
        rwlock_shared_state::RwLockSharedState,
    },
    multipart::item::{image::multipart_image, register::multipart_register},
};
use application::usecase::item::{
    delete::{DeleteItemInputs, DeleteItemOutputs},
    image::{ImageItemInputs, ImageItemOutputs},
    individual::{IndividualItemDataJson, IndividualItemInputs, IndividualItemOutputs},
    register::{RegisterItemInputs, RegisterItemOutputs},
    search::{SearchItemInputs, SearchItemJson, SearchItemOutputs},
    transfer::{TransferItemInputs, TransferItemOutputs},
    trash::{TrashItemDataJson, TrashItemOutputs},
    update::{UpdateItemDataJson, UpdateItemInputs, UpdateItemOutputs},
};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    entity::data_type::transfer_item::TransferItemData,
    value_object::error::{AppError, ResponseError},
};
use std::collections::HashMap;

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
    security(("jwt_token" = [])),
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
    security(("jwt_token" = [])),
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

#[utoipa::path(
    put,
    path = "/api/item/image/{id}",
    tag = "Item",
    params(("id", Path, description = "set item id (not visible id)")),
    request_body(
        description = "ImageItemMultipartData",
        content_type = "multipart/form-data",
        content = ImageItemMultipartData,
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn image_handler(
    Path(id): Path<u32>,
    State(shared_state): State<RwLockSharedState>,
    multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/image handler.");
    tracing::info!("path (id): {}", id);
    // process of multipart/data-form
    let image_item_data_json = multipart_image(multipart).await?;
    let shared_model = shared_state.write().await;
    // operation
    let inputs = ImageItemInputs {
        id,
        image_item_data_json,
    };
    let outputs = ImageItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().image_item,
    )
    .await;
    outputs.run(inputs).await?;
    drop(shared_model);
    Ok((StatusCode::OK, ()).into_response())
}

#[utoipa::path(
    post,
    path = "/api/item/register",
    tag = "Item",
    request_body(
        description = "RegisterItemMultipartData",
        content_type = "multipart/form-data",
        content = RegisterItemMultipartData,
    ),
    responses(
        (status = 201, description = "Created"),
        (status = 400, description = "Bad Request", body = ResponseError),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn register_handler(
    State(shared_state): State<RwLockSharedState>,
    multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/register handler.");
    // process of multipart/data-form
    let register_item_data = multipart_register(multipart).await?;
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
    security(("jwt_token" = [])),
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
    security(("jwt_token" = [])),
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
    get,
    path = "/api/item/trash",
    tag = "Item",
    responses(
        (status = 200, description = "OK", body = TrashItemDataJson),
        (status = 500, description = "Internal Server Error", body = ResponseError),
    ),
    security(("jwt_token" = [])),
)]
pub async fn trash_handler(
    State(shared_state): State<RwLockSharedState>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("reached item/trash handler.");
    let shared_model = shared_state.read().await;
    //operation
    let outputs = TrashItemOutputs::new(
        shared_model.clone().healthcheck,
        shared_model.clone().trash_item,
    )
    .await;
    let result = outputs.run().await?;
    drop(shared_model);
    Ok((StatusCode::OK, Json(result)).into_response())
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
    security(("jwt_token" = [])),
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
