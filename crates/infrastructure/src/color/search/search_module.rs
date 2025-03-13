use domain::{
    entity::data_type::{color::ColorData, meilisearch_color::MeilisearchColorData},
    value_object::error::color::search::SearchColorError,
};
use meilisearch_sdk::client::Client;

pub(super) async fn search(
    meilisearch: Client,
    keywords: String,
) -> Result<Vec<ColorData>, SearchColorError> {
    ////* validation *////
    if keywords.chars().count() == 0 {
        return Err(SearchColorError::EmptyKeywordsError);
    }

    ////* operation *////
    //* convert + to half space *//
    let keywords = keywords.replace("+", " ");
    //* get search result from MeiliSearch *//
    let meilisearch_colors: Vec<MeilisearchColorData> = meilisearch
        .index("color")
        .search()
        .with_query(&keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchColorData to SearchColorData *//
    let search_color_data: Vec<ColorData> = meilisearch_colors
        .into_iter()
        .map(|connector| ColorData {
            id: connector.id,
            name: connector.name,
            hex_color_code: connector.hex_color_code,
            status: connector.status,
        })
        .collect();
    Ok(search_color_data)
}
