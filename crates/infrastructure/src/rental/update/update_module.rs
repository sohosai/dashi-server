use chrono::DateTime;
use domain::{
    entity::{
        data_type::{meilisearch_item::MeilisearchItemData, rental::RentalData},
        discord::sender::DiscordWebHookSender,
    },
    value_object::error::{
        critical_incident, discord::collection::DiscordCollection,
        rental::update::UpdateRentalError,
    },
};
use entity::item::Entity as Item;
use meilisearch_sdk::client::Client;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::discord::rental::discord_rental_webhook_sender;

pub(super) async fn update(
    rdb: DatabaseConnection,
    meilisearch: Client,
    rental_data: RentalData,
    connect_discord_rental_webhook: DiscordCollection,
) -> Result<(), UpdateRentalError> {
    ////* validation *////
    //* validation of recipient is not empty *//
    if rental_data.recipient.chars().count() == 0 {
        return Err(UpdateRentalError::RecipientEmptyError);
    }
    //* validation of id is exist *//
    // validation of id is exist in Item Table
    let item_model = match Item::find_by_id(rental_data.id).all(&rdb).await {
        Ok(item_models) => {
            if item_models.len() > 1 {
                // If multiple ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(UpdateRentalError::IdConflictInItemTableError);
            }
            if item_models.is_empty() {
                // If id does not exist
                return Err(UpdateRentalError::IdNotFoundInItemTableError);
            }
            item_models[0].clone()
        }
        Err(e) => return Err(UpdateRentalError::RDBError(e)),
    };
    // validation of id is exist in MeiliSearch
    let filter_query = &format!(r#"id = "{}""#, rental_data.id);
    let meilisearch_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&rental_data.id.to_string())
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
        return Err(UpdateRentalError::IdConflictInMeiliSearchError);
    }
    if meilisearch_item.is_empty() {
        // If visible_id does not exist
        return Err(UpdateRentalError::IdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;

    //* validation of is_rent is true *//
    // validation of is_rent is true in Item Table
    if !item_model.is_rent {
        // If is_rent is false
        return Err(UpdateRentalError::ItemNotRentedInItemTableError);
    }
    // validation of is_rent is true in MeiliSearch
    if !meilisearch_item[0].is_rent {
        // If is_rent is false
        return Err(UpdateRentalError::ItemNotRentedInMeiliSearchError);
    }

    ////* operation *////
    // pre process
    let mut scheduled_replace_at: Option<DateTime<chrono::FixedOffset>> = None;
    if rental_data.scheduled_replace_at.chars().count() > 0 {
        //transparent from string to DateTime<FixedOffset>
        scheduled_replace_at = Some(
            DateTime::parse_from_rfc3339(&rental_data.scheduled_replace_at)
                .map_err(|e| UpdateRentalError::ParseDateTimeError(e.to_string()))?,
        );
    }
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
        is_rent: meilisearch_item[0].is_rent.to_owned(),
        color: meilisearch_item[0].color.to_owned(),
        created_at: meilisearch_item[0].created_at.to_owned(),
        updated_at: meilisearch_item[0].updated_at.to_owned(),
        recipient: rental_data.recipient.to_owned(),
        rental_description: rental_data.rental_description.to_owned(),
        latest_rent_at: meilisearch_item[0].latest_rent_at.to_owned(),
        scheduled_replace_at,
        latest_replace_at: meilisearch_item[0].latest_replace_at.to_owned(),
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
            return Err(UpdateRentalError::MeiliSearchError(e));
        }
    }

    //* update Item Table *//
    let mut active_item_model = item_model.clone().into_active_model();

    active_item_model.recipient = Set(rental_data.recipient.to_owned());
    active_item_model.rental_description = Set(rental_data.rental_description.to_owned());
    active_item_model.scheduled_replace_at = Set(scheduled_replace_at);

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
            return Err(UpdateRentalError::RDBError(e));
        }
    };

    //* Discord Webhook *//
    let sender = DiscordWebHookSender {
        title: "貸し出し内容の更新情報".to_string(),
        description: "以下の物品の貸し出し内容が更新されました。".to_string(),
        color: 0x50e3c1,
        item: updated_item_model.clone(),
        connect_discord_webhook: connect_discord_rental_webhook,
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
) -> Result<(), UpdateRentalError> {
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
            return Err(UpdateRentalError::MeiliSearchError(e));
        }
    };
    tracing::info!("Rollbacked rented item in MeiliSearch (rollback updated item infomation).");
    tracing::debug!("{:#?}", meilisearch_model);
    tracing::warn!("Rollack Summary");
    tracing::warn!("MeiliSearch: Success");
    Ok(())
}
