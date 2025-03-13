use domain::{
    entity::data_type::{meilisearch_color::MeilisearchColorData, update_color::UpdateColorData},
    value_object::error::{color::update::UpdateColorError, critical_incident},
};
use entity::color::{self, Entity as Color};
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub(super) async fn update(
    rdb: DatabaseConnection,
    meilisearch: Client,
    update_color_data: UpdateColorData,
) -> Result<(), UpdateColorError> {
    ////* operation *////
    // get color in Color Table
    let color_model = match Color::find_by_id(update_color_data.id).one(&rdb).await {
        Ok(color_model) => match color_model {
            Some(color_model) => color_model,
            None => return Err(UpdateColorError::IdNotFoundInItemTableError),
        },
        Err(e) => return Err(UpdateColorError::RDBError(e)),
    };
    // update Color Table
    let mut update_color_model = color_model.clone().into_active_model();
    update_color_model.hex_color_code = Set(update_color_data.hex_color_code);
    update_color_model.status = Set(update_color_data.status);
    let updated_color_model = match update_color_model.update(&rdb).await {
        Ok(color_model) => {
            tracing::info!("Updated to Color Table: {:?}", color_model);
            color_model
        }
        Err(e) => {
            tracing::error!("Failed to update color.");
            tracing::error!("{}", e.to_string());
            return Err(UpdateColorError::RDBError(e));
        }
    };
    // update MeiliSearch
    let meilisearch_color_model = MeilisearchColorData {
        id: updated_color_model.id,
        name: updated_color_model.name,
        hex_color_code: updated_color_model.hex_color_code,
        status: updated_color_model.status,
    };
    match meilisearch
        .index("color")
        .add_documents(&[meilisearch_color_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_item_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_item_model);
        }
        Err(e) => {
            tracing::error!("Failed to update meilisearch.");
            // try rollback
            rollback_rdb(&rdb, color_model).await?;
            return Err(UpdateColorError::MeiliSearchError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    color_model: color::Model,
) -> Result<(), UpdateColorError> {
    let mut update_color_model = color_model.clone().into_active_model();

    update_color_model.hex_color_code = Set(color_model.hex_color_code);
    update_color_model.status = Set(color_model.status);

    let color_model = match update_color_model.update(rdb).await {
        Ok(color_model) => color_model,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback updated item in Color Table (rollback updated color infomation)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(UpdateColorError::RDBError(e));
        }
    };

    tracing::info!("Rollbacked registed color in Color Table (rollback updated color infomation).");
    tracing::debug!("{:#?}", color_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}
