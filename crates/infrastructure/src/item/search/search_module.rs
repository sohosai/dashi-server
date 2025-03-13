use domain::{
    entity::data_type::{meilisearch_item::MeilisearchItemData, search_item::SearchItemData},
    value_object::error::item::search::SearchItemError,
};
use meilisearch_sdk::client::Client;

pub(super) async fn search(
    meilisearch: Client,
    keywords: String,
) -> Result<Vec<SearchItemData>, SearchItemError> {
    ////* validation *////
    if keywords.chars().count() == 0 {
        return Err(SearchItemError::EmptyKeywordsError);
    }

    ////* operation *////
    //* convert + to half space *//
    let keywords = keywords.replace("+", " ");
    //* get search result from MeiliSearch *//
    let meilisearch_items: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchItemData to SearchItemData *//
    let search_item_data: Vec<SearchItemData> = meilisearch_items
        .into_iter()
        .map(|item| SearchItemData {
            id: item.id,
            visible_id: item.visible_id,
            record: item.record,
            name: item.name,
            product_number: item.product_number,
            description: item.description,
            purchase_year: item.purchase_year,
            purchase_price: item.purchase_price,
            durability: item.durability,
            is_depreciation: item.is_depreciation,
            connector: item.connector,
            is_rent: item.is_rent,
            color: item.color,
            created_at: item.created_at.to_string(),
            updated_at: item.updated_at.to_string(),
            recipient: item.recipient,
            rental_description: item.rental_description,
            latest_rent_at: item
                .latest_rent_at
                .map(|latest_rent_at| latest_rent_at.to_string()),
            scheduled_replace_at: item
                .scheduled_replace_at
                .map(|scheduled_replace_at| scheduled_replace_at.to_string()),
            latest_replace_at: item
                .latest_replace_at
                .map(|latest_replace_at| latest_replace_at.to_string()),
        })
        .collect();

    Ok(search_item_data)
}
