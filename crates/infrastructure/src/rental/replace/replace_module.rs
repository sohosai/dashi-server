use domain::{
    entity::{
        data_type::meilisearch_item::MeilisearchItemData, discord::sender::DiscordWebHookSender,
    },
    value_object::error::{
        critical_incident, discord::collection::DiscordCollection,
        rental::replace::ReplaceRentalError,
    },
};
use entity::item::Entity as Item;
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::discord::rental::discord_rental_webhook_sender;

pub(super) async fn replace(
    rdb: DatabaseConnection,
    meilisearch: Client,
    id: u32,
    connect_discord_rental_webhook: DiscordCollection,
) -> Result<(), ReplaceRentalError> {
    ////* validation *////
    //* validation of id is exist *//
    // validation of id is exist in Item Table
    let item_model = match Item::find_by_id(id as i32).all(&rdb).await {
        Ok(item_models) => {
            if item_models.len() > 1 {
                // If multiple ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(ReplaceRentalError::IdConflictInItemTableError);
            }
            if item_models.is_empty() {
                // If id does not exist
                return Err(ReplaceRentalError::IdNotFoundInItemTableError);
            }
            item_models[0].clone()
        }
        Err(e) => return Err(ReplaceRentalError::RDBError(e)),
    };
    // validation of id is exist in MeiliSearch
    let filter_query = &format!(r#"id = "{}""#, id);
    let meilisearch_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&id.to_string())
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    if meilisearch_item.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(ReplaceRentalError::IdConflictInMeiliSearchError);
    }
    if meilisearch_item.is_empty() {
        // If visible_id does not exist
        return Err(ReplaceRentalError::IdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;

    //* validation of is_rent is true *//
    // validation of is_rent is true in Item Table
    if !item_model.is_rent {
        // If is_rent is false
        return Err(ReplaceRentalError::ItemNotRentedInItemTableError);
    }
    // validation of is_rent is true in MeiliSearch
    if !meilisearch_item[0].is_rent {
        // If is_rent is false
        return Err(ReplaceRentalError::ItemNotRentedInMeiliSearchError);
    }

    ////* operation *////
    // pre process
    let latest_replace_at = Some(chrono::Utc::now().naive_utc());
    //* update MeiliSearch *//
    let meilisearch_model: MeilisearchItemData = MeilisearchItemData {
        id: meilisearch_item[0].id,
        visible_id: meilisearch_item[0].visible_id.to_owned(),
        record: meilisearch_item[0].record.to_owned(),
        name: meilisearch_item[0].name.to_owned(),
        product_number: meilisearch_item[0].product_number.to_owned(),
        description: meilisearch_item[0].description.to_owned(),
        purchase_year: meilisearch_item[0].purchase_year,
        purchase_price: meilisearch_item[0].purchase_price,
        durability: meilisearch_item[0].durability,
        is_depreciation: meilisearch_item[0].is_depreciation,
        connector: meilisearch_item[0].connector.to_owned(),
        is_rent: false,
        color: meilisearch_item[0].color.to_owned(),
        created_at: meilisearch_item[0].created_at.to_owned(),
        updated_at: meilisearch_item[0].updated_at.to_owned(),
        recipient: "".to_string(),
        rental_description: "".to_string(),
        latest_rent_at: None,
        scheduled_replace_at: None,
        latest_replace_at,
    };
    match meilisearch
        .index("item")
        .add_documents(&[meilisearch_model], Some("id"))
        .await
    {
        Ok(insert_meilisearch_item_model) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", insert_meilisearch_item_model);
        }
        Err(e) => {
            tracing::error!("Failed to update meilisearch.");
            tracing::error!("{}", e.to_string());
            return Err(ReplaceRentalError::MeiliSearchError(e));
        }
    }

    //* update Item Table *//
    let mut active_item_model = item_model.clone().into_active_model();

    active_item_model.is_rent = Set(false);
    active_item_model.recipient = Set("".to_string());
    active_item_model.rental_description = Set("".to_string());
    active_item_model.latest_rent_at = Set(None);
    active_item_model.scheduled_replace_at = Set(None);
    active_item_model.latest_replace_at = Set(latest_replace_at);

    let updated_item_model = match active_item_model.update(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Updated to Item Table: {:?}", item_model);
            item_model
        }
        Err(e) => {
            tracing::error!("Failed to update item.");
            tracing::error!("{}", e.to_string());
            // try rollback
            rollback_meilisearch(meilisearch, meilisearch_item[0].to_owned()).await?;
            return Err(ReplaceRentalError::RDBError(e));
        }
    };

    //* Discord Webhook *//
    let sender = DiscordWebHookSender {
        title: "返却情報".to_string(),
        description: "以下の物品が返却されました。".to_string(),
        color: 0x78e6d0,
        item: updated_item_model.clone(),
        connect_discord_rental_webhook,
    };
    discord_rental_webhook_sender(sender).await?;

    drop(meilisearch);
    drop(meilisearch_item);
    drop(updated_item_model);

    Ok(())
}

async fn rollback_meilisearch(
    meilisearch: Client,
    meilisearch_item: MeilisearchItemData,
) -> Result<(), ReplaceRentalError> {
    let meilisearch_model: MeilisearchItemData = MeilisearchItemData {
        id: meilisearch_item.id,
        visible_id: meilisearch_item.visible_id.to_owned(),
        record: meilisearch_item.record.to_owned(),
        name: meilisearch_item.name.to_owned(),
        product_number: meilisearch_item.product_number.to_owned(),
        description: meilisearch_item.description.to_owned(),
        purchase_year: meilisearch_item.purchase_year,
        purchase_price: meilisearch_item.purchase_price,
        durability: meilisearch_item.durability,
        is_depreciation: meilisearch_item.is_depreciation,
        connector: meilisearch_item.connector.to_owned(),
        is_rent: meilisearch_item.is_rent,
        color: meilisearch_item.color.to_owned(),
        created_at: meilisearch_item.created_at.to_owned(),
        updated_at: meilisearch_item.updated_at.to_owned(),
        recipient: meilisearch_item.recipient.to_owned(),
        rental_description: meilisearch_item.rental_description.to_owned(),
        latest_rent_at: meilisearch_item.latest_rent_at,
        scheduled_replace_at: meilisearch_item.scheduled_replace_at,
        latest_replace_at: meilisearch_item.latest_replace_at,
    };
    let meilisearch_model = match meilisearch
        .index("item")
        .add_documents(&[meilisearch_model], Some("id"))
        .await
    {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed item in MeiliSearch (rollback updated item infomation)."
            );
            tracing::warn!("Rollback Summary");
            tracing::warn!("MeiliSearch: Failed");
            return Err(ReplaceRentalError::MeiliSearchError(e));
        }
    };
    tracing::info!("Rollbacked rented item in MeiliSearch (rollback updated item infomation).");
    tracing::debug!("{:#?}", meilisearch_model);
    tracing::warn!("Rollack Summary");
    tracing::warn!("MeiliSearch: Success");
    Ok(())
}
