use domain::{
    entity::{
        data_type::{meilisearch_item::MeilisearchItemData, register_item::RegisterItemData},
        discord::sender::DiscordWebHookSender,
    },
    value_object::error::{
        critical_incident, discord::collection::DiscordCollection,
        item::register::RegisterItemError,
    },
};
use entity::{
    item::{self, Entity as Item},
    label::Entity as Label,
};
use meilisearch_sdk::client::Client;
use neo4rs::{query, Graph, Node};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{discord::item::discord_item_webhook_sender, object_strage};

pub(super) async fn register(
    rdb: DatabaseConnection,
    graphdb: Graph,
    meilisearch: Client,
    register_item_data: RegisterItemData,
    connect_discord_item_webhook: DiscordCollection,
) -> Result<(), RegisterItemError> {
    ////* validation *////
    //* validation of name is not empty *//
    if register_item_data.name.chars().count() == 0 {
        return Err(RegisterItemError::ItemNameEmptyError);
    }

    //* validation of visible_id is exist in Label Table *//
    let label_model = match Label::find_by_id(register_item_data.visible_id.to_owned())
        .one(&rdb)
        .await?
    {
        Some(label_model) => label_model,
        None => return Err(RegisterItemError::LabelNotFoundError),
    };

    //* validation of visible_id is not exist *//
    // validation of visible_id is not exist in Item Table
    match Item::find()
        .filter(item::Column::VisibleId.eq(register_item_data.visible_id.to_owned()))
        .all(&rdb)
        .await
    {
        Ok(item_models) => {
            if !item_models.is_empty() {
                if item_models.len() > 1 {
                    // If multiple visible_ids already exist
                    //* critical incident *//
                    critical_incident::conflict_error().await;
                    return Err(RegisterItemError::VisibleIdConflictInItemTableError);
                }
                return Err(RegisterItemError::VisibleIdExistInItemTableError);
            }
        }
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    }
    // validation of visible_id is not exist in MeiliSearch
    let filter_query = &format!(
        r#"visible_id = "{}""#,
        register_item_data.visible_id.to_owned()
    );
    let meilisearch_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&register_item_data.visible_id.to_owned())
        .with_filter(filter_query)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    if !meilisearch_item.is_empty() {
        if meilisearch_item.len() > 1 {
            // If multiple visible_ids already exist
            //* critical incident *//
            critical_incident::conflict_error().await;
            return Err(RegisterItemError::VisibleIdConflictInMeiliSerachError);
        }
        return Err(RegisterItemError::VisibleIdExistInMeiliSerachError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;

    //* validation of parent_visible_id is exist *//
    // validation of parent_viible_id is exist in Item Table.
    let parent_item_model = match Item::find()
        .filter(item::Column::VisibleId.eq(register_item_data.parent_visible_id.to_owned()))
        .all(&rdb)
        .await
    {
        Ok(item_models) => {
            if item_models.is_empty() {
                return Err(RegisterItemError::ParentVisibleIdNotFoundInItemTableError);
            }
            if item_models.len() > 1 {
                // If multiple visible_ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(RegisterItemError::VisibleIdConflictInItemTableError);
            }
            item_models[0].to_owned()
        }
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    };
    // validation of parent_viible_id is exist in MeiliSearch.
    let filter_query = &format!(
        r#"visible_id = "{}""#,
        parent_item_model.visible_id.to_owned()
    );
    let meilisearch_item: Vec<MeilisearchItemData> = meilisearch
        .index("item")
        .search()
        .with_query(&parent_item_model.visible_id.to_owned())
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
        return Err(RegisterItemError::VisibleIdConflictInMeiliSerachError);
    }
    if meilisearch_item.is_empty() {
        return Err(RegisterItemError::ParentVisibleIdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;

    // validation of parent_viible_id is exist in GraphDB.
    let mut item_node = graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", parent_item_model.id))
        .await?;
    // parse node
    let mut item_nodes: Vec<i64> = Vec::new();
    loop {
        let item = match item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(RegisterItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                return Err(RegisterItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(RegisterItemError::GraphDBDeError(e));
            }
        };
        item_nodes.push(id);
    }
    if item_nodes.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(RegisterItemError::VisibleIdConflictInGraphDBError);
    }
    if item_nodes.is_empty() {
        // If visible_id does not exist
        return Err(RegisterItemError::VisibleIdNotFoundInGraphDBError);
    }

    ////* operation *////
    //* insert to RDB *//
    let item_model = item::ActiveModel {
        visible_id: Set(register_item_data.visible_id.to_owned()),
        name: Set(register_item_data.name.to_owned()),
        product_number: Set(register_item_data.product_number.to_owned()),
        description: Set(register_item_data.description.to_owned()),
        purchase_year: Set(register_item_data.purchase_year),
        purchase_price: Set(register_item_data.purchase_price),
        durability: Set(register_item_data.durability),
        is_depreciation: Set(register_item_data.is_depreciation),
        connector: Set(serde_json::json!(register_item_data.connector)),
        is_rent: Set(false),
        color: Set(register_item_data.color.to_owned()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        recipient: Set("".to_owned()),
        rental_description: Set("".to_owned()),
        latest_rent_at: Set(None),
        scheduled_replace_at: Set(None),
        latest_replace_at: Set(None),
        ..Default::default()
    };
    let inserted_item_model = match Item::insert(item_model).exec(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Inserted to Item Table: {:?}", item_model);
            item_model
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            tracing::error!("{}", e.to_string());
            return Err(RegisterItemError::RDBError(e));
        }
    };
    //get registered item
    let registered_item_model = match Item::find_by_id(inserted_item_model.last_insert_id)
        .one(&rdb)
        .await
    {
        Ok(item_model) => match item_model {
            Some(item_model) => item_model,
            None => return Err(RegisterItemError::RegisteredItemNotFoundError),
        },
        Err(e) => return Err(RegisterItemError::RDBError(e)),
    };

    //* upload image to R2 *//
    match object_strage::r2::upload(registered_item_model.id, &register_item_data.image).await {
        Ok(_) => {
            tracing::info!("Uploaded image to R2.");
        }
        Err(e) => {
            tracing::error!("Failed to upload image to R2.");
            // try rollback
            rollback_rdb(&rdb, registered_item_model).await?;
            return Err(RegisterItemError::ObjectStrageError(e));
        }
    };

    //* insert to meilisearch *//
    let meilisearch_model: MeilisearchItemData = MeilisearchItemData {
        id: registered_item_model.id,
        visible_id: registered_item_model.visible_id.to_owned(),
        record: label_model.record,
        name: registered_item_model.name.to_owned(),
        product_number: registered_item_model.product_number.to_owned(),
        description: registered_item_model.description.to_owned(),
        purchase_year: registered_item_model.purchase_year,
        purchase_price: registered_item_model.purchase_price,
        durability: registered_item_model.durability,
        is_depreciation: registered_item_model.is_depreciation,
        connector: register_item_data.connector.to_owned(),
        is_rent: false,
        color: registered_item_model.color.to_owned(),
        created_at: registered_item_model.created_at.to_owned(),
        updated_at: registered_item_model.updated_at.to_owned(),
        recipient: "".to_string(),
        rental_description: "".to_string(),
        latest_rent_at: None,
        scheduled_replace_at: None,
        latest_replace_at: None,
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
            tracing::error!("Failed to insert meilisearch.");
            // try rollback
            rollback_rdb_object_strage(&rdb, registered_item_model).await?;
            return Err(RegisterItemError::MeiliSearchError(e));
        }
    }

    //* insert to GraphDB *//
    match graphdb
        .run(
            query(
                "MATCH (parent:Item {id: $parent_id}) CREATE (child:Item {id: $child_id})-[relation:Parent]->(parent)"
            )
            .param("parent_id", parent_item_model.id)
            .param("child_id", registered_item_model.id)
        )
        .await
    {
        Ok(_) => {
            tracing::info!("Inserted to GraphDB");
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            // try rollback
            rollback_rdb_object_strage_meilisearch(&rdb, meilisearch, registered_item_model).await?;
            return Err(RegisterItemError::GraphDBError(e));
        }
    }

    //* Discord Webhook *//
    let sender = DiscordWebHookSender {
        title: "物品の登録情報".to_string(),
        description: "以下の物品が登録されました。".to_string(),
        color: 0x49cb90,
        item: registered_item_model.clone(),
        connect_discord_webhook: connect_discord_item_webhook,
    };
    discord_item_webhook_sender(sender).await?;

    Ok(())
}

// Rollback functions
async fn rollback_rdb_object_strage_meilisearch(
    rdb: &DatabaseConnection,
    meilisearch: Client,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    // rollback RDB
    match rollback_rdb(rdb, registered_item_model.to_owned()).await {
        Ok(_) => {
            // Rollbacked RDB: Success
            // rollback MeiliSearch
            match rollback_meilisearch(meilisearch, registered_item_model.to_owned()).await {
                Ok(_) => {
                    // Rollbacked RDB: Success, Rollbacked MeiliSearch: Success
                    match rollback_object_strage(registered_item_model.id).await {
                        Ok(_) => {
                            // Rollbacked RDB: Success, Rollbacked MeiliSearch: Success, Rollbacked ObjectStrage: Success
                        }
                        Err(e) => {
                            // Rollbacked RDB: Success, Rollbacked MeiliSearch: Success, Rollbacked ObjectStrage: Failed
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    // Rollbacked RDB: Success, Rollbacked MeiliSearch: Failed
                    match rollback_object_strage(registered_item_model.id).await {
                        Ok(_) => {
                            // Rollbacked RDB: Success, Rollbacked MeiliSearch: Failed, Rollbacked ObjectStrage: Success
                        }
                        Err(e) => {
                            // Rollbacked RDB: Success, Rollbacked MeiliSearch: Failed, Rollbacked ObjectStrage: Failed
                            return Err(e);
                        }
                    }
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // Rollbacked RDB: Failed
            // rollback MeiliSearch
            match rollback_meilisearch(meilisearch, registered_item_model.to_owned()).await {
                Ok(_) => {
                    // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Success
                    match rollback_object_strage(registered_item_model.id).await {
                        Ok(_) => {
                            // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Success, Rollbacked ObjectStrage: Success
                        }
                        Err(e) => {
                            // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Success, Rollbacked ObjectStrage: Failed
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Failed
                    match rollback_object_strage(registered_item_model.id).await {
                        Ok(_) => {
                            // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Failed, Rollbacked ObjectStrage: Success
                        }
                        Err(e) => {
                            // Rollbacked RDB: Failed, Rollbacked MeiliSearch: Failed, Rollbacked ObjectStrage: Failed
                            return Err(e);
                        }
                    }
                    return Err(e);
                }
            }
            return Err(e);
        }
    }
    Ok(())
}

async fn rollback_rdb_object_strage(
    rdb: &DatabaseConnection,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    match rollback_rdb(rdb, registered_item_model.to_owned()).await {
        Ok(_) => {
            // Rollbacked RDB: Success
            match rollback_object_strage(registered_item_model.id).await {
                Ok(_) => {
                    // Rollbacked RDB: Success, Rollbacked ObjectStrage: Success
                }
                Err(e) => {
                    // Rollbacked RDB: Success, Rollbacked ObjectStrage: Failed
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // Rollbacked RDB: Failed
            match rollback_object_strage(registered_item_model.id).await {
                Ok(_) => {
                    // Rollbacked RDB: Failed, Rollbacked ObjectStrage: Success
                }
                Err(e) => {
                    // Rollbacked RDB: Failed, Rollbacked ObjectStrage: Failed
                    return Err(e);
                }
            }
            return Err(e);
        }
    }
    Ok(())
}

async fn rollback_rdb(
    rdb: &DatabaseConnection,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    let item_model = match registered_item_model.into_active_model().delete(rdb).await {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed item in Item Table (delete registered item)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("RDB: Failed");
            return Err(RegisterItemError::RDBError(e));
        }
    };
    tracing::info!("Rollbacked registed item in Item Table (delete registered item).");
    tracing::debug!("{:#?}", item_model);
    tracing::warn!("Rollback Summary");
    tracing::warn!("RDB: Success");
    Ok(())
}

async fn rollback_meilisearch(
    meilisearch: Client,
    registered_item_model: item::Model,
) -> Result<(), RegisterItemError> {
    let meilisearch_model = meilisearch
        .index("item")
        .delete_document(registered_item_model.id)
        .await;
    let meilisearch_model = match meilisearch_model {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registed item in MeiliSearch (delete registered item)."
            );
            tracing::warn!("Rollback Summary");
            tracing::warn!("MeiliSearch: Failed");
            return Err(RegisterItemError::MeiliSearchError(e));
        }
    };
    tracing::info!("Rollbacked registed item in MeiliSearch (delete registered item).");
    tracing::debug!("{:#?}", meilisearch_model);
    tracing::warn!("Rollack Summary");
    tracing::warn!("MeiliSearch: Success");
    Ok(())
}

async fn rollback_object_strage(id: i32) -> Result<(), RegisterItemError> {
    match object_strage::r2::delete(id).await {
        Ok(_) => {}
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback registered item in Object Strage (delete registered item infomation)."
            );
            tracing::warn!("Rollbaack Summary");
            tracing::warn!("Object Strage: Failed");
            return Err(RegisterItemError::ObjectStrageError(e));
        }
    };

    tracing::info!("Rollbacked registered item in Item Table (delete registered item infomation).");
    tracing::warn!("Rollback Summary");
    tracing::warn!("Object Strage: Success");
    Ok(())
}
