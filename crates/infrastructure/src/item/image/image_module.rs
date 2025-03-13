use domain::{
    entity::data_type::{image_item::ImageItemData, meilisearch_item::MeilisearchItemData},
    value_object::error::{critical_incident, item::image::ImageItemError},
};
use entity::item::Entity as Item;
use meilisearch_sdk::client::Client;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::object_strage;

pub(super) async fn image(
    rdb: DatabaseConnection,
    meilisearch: Client,
    image_item_data: ImageItemData,
) -> Result<(), ImageItemError> {
    ////* validation *////
    //* validation of id is exist *//
    // validation of id is exist in Item Table
    match Item::find_by_id(image_item_data.id).all(&rdb).await {
        Ok(item_models) => {
            if item_models.len() > 1 {
                // If multiple ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(ImageItemError::IdConflictInItemTableError);
            }
            if item_models.is_empty() {
                // If id does not exist
                return Err(ImageItemError::IdNotFoundInItemTableError);
            }
            item_models[0].clone()
        }
        Err(e) => return Err(ImageItemError::RDBError(e)),
    };
    // validation of id is exist in MeiliSearch
    let filter_query = &format!(r#"id = "{}""#, (image_item_data.id));
    let meilisearch_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&image_item_data.id.to_string())
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    if meilisearch_item.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(ImageItemError::IdConflictInMeiliSerachError);
    }
    if meilisearch_item.is_empty() {
        // If visible_id does not exist
        return Err(ImageItemError::IdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;

    ////* operation *////
    //* upload image to R2 *//
    match object_strage::r2::upload(image_item_data.id, &image_item_data.image).await {
        Ok(_) => {
            tracing::info!("Uploaded image to R2.");
        }
        Err(e) => {
            tracing::error!("Failed to upload image to R2.");
            return Err(ImageItemError::ObjectStrageError(e));
        }
    };
    Ok(())
}
