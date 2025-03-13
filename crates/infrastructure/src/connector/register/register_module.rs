use domain::{
    entity::data_type::{
        meilisearch_connector::MeilisearchConnectorData, register_connector::RegisterConnectorData,
    },
    value_object::error::{connector::register::RegisterConnectorError, critical_incident},
};
use entity::{
    active_enum::Status,
    connector::{self, Entity as Connector},
};
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub(super) async fn register(
    rdb: DatabaseConnection,
    meilisearch: Client,
    register_connector_data: RegisterConnectorData,
) -> Result<(), RegisterConnectorError> {
    ////* validation *////
    //* validation of name is unique in Connctor Table *//
    let connector_models = Connector::find().all(&rdb).await?;
    for connector_model in connector_models {
        if connector_model.name == register_connector_data.name {
            return Err(RegisterConnectorError::ConflictConnectorNameError);
        }
    }

    ////* operation *////
    //* insert to RDB *//
    let connector_model = connector::ActiveModel {
        name: Set(register_connector_data.name.to_owned()),
        status: Set(Status::Active),
        ..Default::default()
    };
    let inserted_connector_model = match Connector::insert(connector_model).exec(&rdb).await {
        Ok(connector_model) => {
            tracing::info!("Inserted to Connector Table: {:?}", connector_model);
            connector_model
        }
        Err(e) => return Err(RegisterConnectorError::RDBError(e)),
    };
    // get inserted connector
    let registered_connector_model =
        match Connector::find_by_id(inserted_connector_model.last_insert_id)
            .one(&rdb)
            .await
        {
            Ok(connector_model) => match connector_model {
                Some(connector_model) => connector_model,
                None => return Err(RegisterConnectorError::RegisteredConnectorNotFoundError),
            },
            Err(e) => return Err(RegisterConnectorError::RDBError(e)),
        };

    //* insert to MeiliSearch *//
    let meilisearch_connector_model = MeilisearchConnectorData {
        id: registered_connector_model.id,
        name: registered_connector_model.name.to_owned(),
        status: registered_connector_model.status.to_owned(),
    };
    match meilisearch
        .index("connector")
        .add_documents(&[meilisearch_connector_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_connector_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_connector_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert meilisearch.");
            // try rollback
            rollback_rdb(&rdb, registered_connector_model).await?;
            return Err(RegisterConnectorError::MeiliSearchError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    registered_connector_model: connector::Model,
) -> Result<(), RegisterConnectorError> {
    let connector_model = match registered_connector_model
        .into_active_model()
        .delete(rdb)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed connector in Connector Table (delete registered connector)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(RegisterConnectorError::RDBError(e));
        }
    };
    tracing::info!(
        "Rollbacked registed connector in Connector Table (delete registered connector)."
    );
    tracing::debug!("{:#?}", connector_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}
