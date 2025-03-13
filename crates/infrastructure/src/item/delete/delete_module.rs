use domain::{
    entity::data_type::meilisearch_item::MeilisearchItemData,
    value_object::error::{critical_incident, item::delete::DeleteItemError},
};
use entity::{
    item::Entity as Item,
    trash::{self, Entity as Trash},
};
use meilisearch_sdk::client::Client;
use neo4rs::{query, Graph, Node};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub(super) async fn delete(
    rdb: DatabaseConnection,
    graphdb: Graph,
    meilisearch: Client,
    id: u32,
) -> Result<(), DeleteItemError> {
    ////* validation *////
    //* validation of id is not 1 *//
    // validation of id is not 1 in Item Table
    if id == 1 {
        // If id is 1
        return Err(DeleteItemError::CannotDeleteRootItemError);
    }

    //* validation of id is exist *//
    // validation of id is exist in Item Table
    let item_model = match Item::find_by_id(id as i32).all(&rdb).await {
        Ok(item_models) => {
            if item_models.len() > 1 {
                // If multiple ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(DeleteItemError::IdConflictInItemTableError);
            }
            if item_models.is_empty() {
                // If id does not exist
                return Err(DeleteItemError::IdNotFoundInItemTableError);
            }
            item_models[0].clone()
        }
        Err(e) => return Err(DeleteItemError::RDBError(e)),
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
        return Err(DeleteItemError::IdConflictInMeiliSerachError);
    }
    if meilisearch_item.is_empty() {
        // If visible_id does not exist
        return Err(DeleteItemError::IdNotFoundInMeiliSearchError);
    }
    //drop filter_query and meilisearch_item
    let _ = filter_query;
    let _ = meilisearch_item;

    //* validation of id is exist and is leaf *//
    // validation of id is exist and is leaf in GraphDB
    let mut item_node = graphdb
        .execute(
            query("MATCH (item:Item {id: $id}) WHERE NOT exists(()-[:Parent]->(item)) RETURN item")
                .param("id", id),
        )
        .await?;
    // parse node
    let mut item_nodes: Vec<i64> = Vec::new();
    loop {
        let item = match item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(DeleteItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                return Err(DeleteItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(DeleteItemError::GraphDBDeError(e));
            }
        };
        item_nodes.push(id);
    }
    if item_nodes.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(DeleteItemError::VisibleIdConflictInGraphDBError);
    }
    if item_nodes.is_empty() {
        // If visible_id does not exist
        return Err(DeleteItemError::VisibleIdNotFoundInGraphDBError);
    }
    //drop item_node and item_node
    let _ = item_node;
    let _ = item_nodes;

    //* validation of is_rent is false *//
    // validation of is_rent is false in Item Table
    if item_model.is_rent {
        // If is_rent is true
        return Err(DeleteItemError::ItemOnLoanError);
    }

    ////* operation *////
    //* delete item from MeiliSerach *//
    match meilisearch
        .index("item")
        .delete_document(item_model.id)
        .await
    {
        Ok(task) => {
            tracing::info!("MeiliSearch result of item.");
            tracing::info!("{:#?}", task);
        }
        Err(e) => return Err(DeleteItemError::MeiliSearchError(e)),
    }
    //* delete item from GraphDB (Item Table) *//
    match graphdb
        .run(
            query("MATCH (item:Item {id: $delete_id}) DETACH DELETE item")
                .param("delete_id", item_model.id),
        )
        .await
    {
        Ok(_) => {
            tracing::info!("Deleted from GraphDB");
        }
        Err(e) => {
            tracing::error!("Failed to delete item.");
            // try rollback
            rollback_meilisearch(meilisearch, meilisearch_item).await?;
            return Err(DeleteItemError::GraphDBError(e));
        }
    }
    //* delete item from RDB *//
    match item_model.clone().into_active_model().delete(&rdb).await {
        Ok(item_model) => {
            tracing::info!("RDB result of item.");
            tracing::info!("{:#?}", item_model);
        }
        Err(e) => {
            tracing::error!("Failed to delete RDB.");
            // try rollback
            rollback_meilisearch_graphdb(
                meilisearch,
                graphdb,
                meilisearch_item,
                item_model.id as i64,
            )
            .await?;
            return Err(DeleteItemError::RDBError(e));
        }
    }

    //* add item from RDB (Trash Table) *//
    let trash_model = trash::ActiveModel {
        item_id: Set(item_model.id),
        visible_id: Set(item_model.visible_id.to_owned()),
        name: Set(item_model.name.to_owned()),
        product_number: Set(item_model.product_number.to_owned()),
        description: Set(item_model.description.to_owned()),
        purchase_year: Set(item_model.purchase_year),
        purchase_price: Set(item_model.purchase_price),
        durability: Set(item_model.durability),
        is_depreciation: Set(item_model.is_depreciation),
        connector: Set(item_model.connector),
        is_rent: Set(item_model.is_rent),
        color: Set(item_model.color.to_owned()),
        created_at: Set(item_model.created_at),
        updated_at: Set(item_model.updated_at),
        ..Default::default()
    };
    match Trash::insert(trash_model).exec(&rdb).await {
        Ok(trash_model) => {
            tracing::info!("RDB result of item.");
            tracing::info!("{:#?}", trash_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert RDB.");
            return Err(DeleteItemError::RDBError(e));
        }
    }

    Ok(())
}

async fn rollback_meilisearch_graphdb(
    meilisearch: Client,
    graphdb: Graph,
    meilisearch_item: Vec<MeilisearchItemData>,
    deleted_item_id: i64,
) -> Result<(), DeleteItemError> {
    // rollback MeiliSearch
    match rollback_meilisearch(meilisearch, meilisearch_item).await {
        Ok(_) => {
            // Rollbacked MeiliSearch: Success
            // rollback GraphDB
            match rollback_graphdb(graphdb, deleted_item_id).await {
                Ok(_) => {
                    // Rollbacked MeiliSearch: Success, Rollbacked GraphDB: Success
                }
                Err(e) => {
                    // Rollbacked MeiliSearch: Success, Rollbacked GraphDB: Failed
                    return Err(e);
                }
            }
        }
        Err(e) => {
            // Rollbacked MeiliSearch: Failed
            // rollback GraphDB
            match rollback_graphdb(graphdb, deleted_item_id).await {
                Ok(_) => {
                    // Rollbacked MeiliSearch: Failed, Rollbacked GraphDB: Success
                }
                Err(e) => {
                    // Rollbacked MeiliSearch: Failed, Rollbacked GraphDB: Failed
                    return Err(e);
                }
            }
            return Err(e);
        }
    }
    Ok(())
}

async fn rollback_meilisearch(
    meilisearch: Client,
    meilisearch_item: Vec<MeilisearchItemData>,
) -> Result<(), DeleteItemError> {
    let meilisearch_model = match meilisearch
        .index("item")
        .add_documents(&meilisearch_item, Some("id"))
        .await
    {
        Ok(result) => result,
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback deleted item in MeiliSearch (restore delete item)."
            );
            tracing::warn!("Rollback Summary");
            tracing::warn!("MeiliSearch: Failed");
            return Err(DeleteItemError::MeiliSearchError(e));
        }
    };
    tracing::info!("Rollbacked deleted item in MeiliSearch (restore delete item).");
    tracing::debug!("{:#?}", meilisearch_model);
    tracing::warn!("Rollack Summary");
    tracing::warn!("MeiliSearch: Success");
    Ok(())
}

async fn rollback_graphdb(graphdb: Graph, deleted_item_id: i64) -> Result<(), DeleteItemError> {
    //* get parent_item_id *//
    let mut parent_item_node = match graphdb
        .execute(
            query("MATCH (:Item {id: $child_id})-[:Parent]->(parent) RETURN parent")
                .param("child_id", deleted_item_id),
        )
        .await
    {
        Ok(parent_item_id) => parent_item_id,
        Err(e) => return Err(DeleteItemError::GraphDBError(e)),
    };
    let mut parent_item_nodes: Vec<i64> = Vec::new();
    loop {
        let item = match parent_item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(DeleteItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                return Err(DeleteItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(DeleteItemError::GraphDBDeError(e));
            }
        };
        parent_item_nodes.push(id);
    }
    if parent_item_nodes.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::multiple_parent_items_error().await;
        return Err(DeleteItemError::MultipleParentItemsError);
    }
    if parent_item_nodes.is_empty() {
        // If visible_id does not exist
        //* critical incident *//
        critical_incident::parent_item_missing_error().await;
        return Err(DeleteItemError::ParentItemMissingError);
    }
    //drop item_node and item_node
    let _ = parent_item_node;
    let _ = parent_item_nodes;
    // get parent_item_id
    let parent_item_id = parent_item_nodes[0];

    //* rollback delete item *//
    match graphdb
        .run(
            query(
                "MATCH (parent:Item {id: $parent_id}) CREATE (child:Item {id: $child_id})-[relation:Parent]->(parent)"
            )
            .param("parent_id", parent_item_id)
            .param("child_id", deleted_item_id)
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            critical_incident::rollback_error().await;
            tracing::error!(
                "Failed to rollback deleted item in GraphDB (restore delete item)."
            );
            tracing::warn!("Rollback Summary");
            tracing::warn!("GraphDB: Failed");
            return Err(DeleteItemError::GraphDBError(e));
        }
    }
    tracing::info!("Rollbacked deleted item in GraphDB (restore delete item).");
    tracing::warn!("Rollack Summary");
    tracing::warn!("GraphDB: Success");
    Ok(())
}
