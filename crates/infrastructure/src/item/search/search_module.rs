use domain::{
    entity::data_type::{meilisearch::MeilisearchData, search_item::SearchItemData},
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
    let meilisearch_items: Vec<MeilisearchData> = meilisearch
        .index("item")
        .search()
        .with_query(&keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchData to SearchItemData *//
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
        })
        .collect();

    Ok(search_item_data)
}

// pub id: i32,
// pub visible_id: String,
// pub record: entity::label::Record,
// pub name: String,
// pub product_number: String,
// pub description: String,
// pub purchase_year: Option<i32>,
// pub purchase_price: Option<i32>,
// pub durability: Option<i32>,
// pub is_depreciation: bool,
// pub connector: Vec<String>,
// pub is_rent: bool,
// pub color: String,
// pub created_at: String,
// pub updated_at: String,
