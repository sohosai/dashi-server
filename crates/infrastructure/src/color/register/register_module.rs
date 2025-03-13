use domain::{
    entity::data_type::{
        meilisearch_color::MeilisearchColorData, register_color::RegisterColorData,
    },
    value_object::error::{color::register::RegisterColorError, critical_incident},
};
use entity::{
    active_enum::Status,
    color::{self, Entity as Color},
};
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub(super) async fn register(
    rdb: DatabaseConnection,
    meilisearch: Client,
    register_color_data: RegisterColorData,
) -> Result<(), RegisterColorError> {
    ////* validation *////
    //* validation of name is unique in Color Table *//
    let color_models = Color::find().all(&rdb).await?;
    for color_model in color_models {
        if color_model.name == register_color_data.name {
            return Err(RegisterColorError::ConflictColorNameError);
        }
    }

    ////* operation *////
    //* insert to RDB *//
    let color_model = color::ActiveModel {
        name: Set(register_color_data.name.to_owned()),
        hex_color_code: Set(register_color_data.hex_color_code.to_owned()),
        status: Set(Status::Active),
        ..Default::default()
    };
    let inserted_color_model = match Color::insert(color_model).exec(&rdb).await {
        Ok(color_model) => {
            tracing::info!("Inserted to Color Table: {:?}", color_model);
            color_model
        }
        Err(e) => return Err(RegisterColorError::RDBError(e)),
    };
    // get inserted color
    let registered_color_model = match Color::find_by_id(inserted_color_model.last_insert_id)
        .one(&rdb)
        .await
    {
        Ok(color_model) => match color_model {
            Some(color_model) => color_model,
            None => return Err(RegisterColorError::RegisteredColorNotFoundError),
        },
        Err(e) => return Err(RegisterColorError::RDBError(e)),
    };

    //* insert to MeiliSearch *//
    let meilisearch_color_model = MeilisearchColorData {
        id: registered_color_model.id,
        name: registered_color_model.name.to_owned(),
        hex_color_code: registered_color_model.hex_color_code.to_owned(),
        status: registered_color_model.status.to_owned(),
    };
    match meilisearch
        .index("color")
        .add_documents(&[meilisearch_color_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_color_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_color_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert meilisearch.");
            // try rollback
            rollback_rdb(&rdb, registered_color_model).await?;
            return Err(RegisterColorError::MeiliSearchError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    registered_color_model: color::Model,
) -> Result<(), RegisterColorError> {
    let color_model = match registered_color_model.into_active_model().delete(rdb).await {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed color in Color Table (delete registered color)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(RegisterColorError::RDBError(e));
        }
    };
    tracing::info!("Rollbacked registed color in Color Table (delete registered color).");
    tracing::debug!("{:#?}", color_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}
