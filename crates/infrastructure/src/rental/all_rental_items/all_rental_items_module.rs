use domain::{
    entity::data_type::{meilisearch_item::MeilisearchItemData, rental_item::RentalItemData},
    value_object::error::item::search::SearchItemError,
};
use meilisearch_sdk::client::Client;

pub(super) async fn all_rental_items(
    meilisearch: Client,
) -> Result<Vec<RentalItemData>, SearchItemError> {
    ////* operation *////
    //* get search result from MeiliSearch *//
    let filter_query = &format!(r#"is_rent = "{}""#, true);
    let meilisearch_items: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchItemData to SearchItemData *//
    let rental_item_data: Vec<RentalItemData> = meilisearch_items
        .into_iter()
        .map(|item| RentalItemData {
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

    Ok(rental_item_data)
}
