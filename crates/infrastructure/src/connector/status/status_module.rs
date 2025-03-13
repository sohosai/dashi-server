use domain::{
    entity::data_type::{
        meilisearch_connector::MeilisearchConnectorData, status_connector::StatusConnectorData,
    },
    value_object::error::{connector::status::StatusConnectorError, critical_incident},
};
use entity::connector::{self, Entity as Connector};
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub(super) async fn status(
    rdb: DatabaseConnection,
    meilisearch: Client,
    status_connector_data: StatusConnectorData,
) -> Result<(), StatusConnectorError> {
    ////* operation *////
    // get connector in Connector Table
    let connector_model = match Connector::find_by_id(status_connector_data.id)
        .one(&rdb)
        .await
    {
        Ok(connector_model) => match connector_model {
            Some(connector_model) => connector_model,
            None => return Err(StatusConnectorError::IdNotFoundInItemTableError),
        },
        Err(e) => return Err(StatusConnectorError::RDBError(e)),
    };
    // update Connector Table
    let mut status_connector_model = connector_model.clone().into_active_model();
    status_connector_model.status = Set(status_connector_data.status);
    let updated_connector_model = match status_connector_model.update(&rdb).await {
        Ok(connector_model) => {
            tracing::info!("Updated to Connector Table: {:?}", connector_model);
            connector_model
        }
        Err(e) => {
            tracing::error!("Failed to update connector.");
            tracing::error!("{}", e.to_string());
            return Err(StatusConnectorError::RDBError(e));
        }
    };
    // update MeiliSearch
    let meilisearch_connector_model = MeilisearchConnectorData {
        id: updated_connector_model.id,
        name: updated_connector_model.name,
        status: updated_connector_model.status,
    };
    match meilisearch
        .index("connector")
        .add_documents(&[meilisearch_connector_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_item_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_item_model);
        }
        Err(e) => {
            tracing::error!("Failed to update meilisearch.");
            // try rollback
            rollback_rdb(&rdb, connector_model).await?;
            return Err(StatusConnectorError::MeiliSearchError(e));
        }
    }

    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    connector_model: connector::Model,
) -> Result<(), StatusConnectorError> {
    let mut status_connector_model = connector_model.clone().into_active_model();

    status_connector_model.status = Set(connector_model.status);

    let connector_model = match status_connector_model.update(rdb).await {
        Ok(connector_model) => connector_model,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback updated item in Connector Table (rollback updated connector infomation)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(StatusConnectorError::RDBError(e));
        }
    };

    tracing::info!(
        "Rollbacked registed connector in Connector Table (rollback updated connector infomation)."
    );
    tracing::debug!("{:#?}", connector_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}
