use domain::{
    entity::data_type::{
        connector::ConnectorData, meilisearch_connector::MeilisearchConnectorData,
    },
    value_object::error::connector::search::SearchConnectorError,
};
use meilisearch_sdk::client::Client;

pub(super) async fn search(
    meilisearch: Client,
    keywords: String,
) -> Result<Vec<ConnectorData>, SearchConnectorError> {
    ////* validation *////
    if keywords.chars().count() == 0 {
        return Err(SearchConnectorError::EmptyKeywordsError);
    }

    ////* operation *////
    //* convert + to half space *//
    let keywords = keywords.replace("+", " ");
    //* get search result from MeiliSearch *//
    let meilisearch_connectors: Vec<MeilisearchConnectorData> = meilisearch
        .index("connector")
        .search()
        .with_query(&keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchConnectorData to SearchConnectorData *//
    let search_connector_data: Vec<ConnectorData> = meilisearch_connectors
        .into_iter()
        .map(|connector| ConnectorData {
            id: connector.id,
            name: connector.name,
            status: connector.status,
        })
        .collect();
    Ok(search_connector_data)
}
