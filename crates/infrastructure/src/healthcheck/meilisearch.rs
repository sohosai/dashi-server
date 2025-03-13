use domain::{
    entity::data_type::meilisearch_item::MeilisearchItemData,
    value_object::error::healthcheck::HealthCheckError,
};
use entity::active_enum::Record;
use meilisearch_sdk::client::Client;

pub(super) async fn healthcheck_meilisearch(meilisearch: Client) -> Result<(), HealthCheckError> {
    //* test *//
    let _ = meilisearch.health().await?;

    //* check *//
    let filter_query = &format!(r#"id = "{}""#, 1);
    let root_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query("1")
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    if root_item.is_empty() {
        return Err(HealthCheckError::RootItemNotFoundError);
    }
    let connector: Vec<String> = Vec::new();
    let correct_root_item = MeilisearchItemData {
        id: 1,
        visible_id: "0000".to_string(),
        record: Record::Nothing,
        name: "筑波大学".to_string(),
        product_number: "".to_string(),
        description: "根の物品です。".to_string(),
        purchase_year: None,
        purchase_price: None,
        durability: None,
        is_depreciation: false,
        connector,
        is_rent: root_item[0].is_rent,
        color: "".to_string(),
        created_at: root_item[0].created_at,
        updated_at: root_item[0].updated_at,
        recipient: root_item[0].recipient.to_owned(),
        rental_description: root_item[0].rental_description.to_owned(),
        latest_rent_at: root_item[0].latest_rent_at,
        scheduled_replace_at: root_item[0].scheduled_replace_at,
        latest_replace_at: root_item[0].latest_replace_at,
    };

    for item in root_item {
        if item != correct_root_item {
            return Err(HealthCheckError::IncompatibleInMeiliSearchError);
        }
    }

    tracing::info!("MeiliSearch is healthy.");
    Ok(())
}
