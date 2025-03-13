use domain::{
    entity::data_type::connector::ConnectorData,
    value_object::error::connector::all_connectors::AllConnectorsError,
};
use entity::connector::Entity as Connector;
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn all_connectors(
    rdb: DatabaseConnection,
) -> Result<Vec<ConnectorData>, AllConnectorsError> {
    let all_connectors_data: Vec<ConnectorData> = Connector::find()
        .all(&rdb)
        .await?
        .into_iter()
        .map(|item| ConnectorData {
            id: item.id,
            name: item.name,
            status: item.status,
        })
        .collect();
    Ok(all_connectors_data)
}
