use crate::discord::item::discord_item_webhook_sender;
use domain::{
    entity::{data_type::transfer_item::TransferItemData, discord::sender::DiscordWebHookSender},
    value_object::error::{
        critical_incident, discord::collection::DiscordCollection,
        item::transfer::TransferItemError,
    },
};
use entity::item::Entity as Item;
use neo4rs::{query, Graph, Node};
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::HashSet;

pub(super) async fn transfer(
    rdb: DatabaseConnection,
    graphdb: Graph,
    transfer_item_data: TransferItemData,
    connect_discord_item_webhook: DiscordCollection,
) -> Result<(), TransferItemError> {
    ////* validation *////
    //* validation of id is not root item *//
    if transfer_item_data.id == 1 {
        return Err(TransferItemError::CannotTransferRootItemError);
    }
    //* validation of id is exist in GraphDB *//
    // get item node
    let mut item_node = graphdb
        .execute(
            query("MATCH (item:Item {id: $id}) RETURN item").param("id", transfer_item_data.id),
        )
        .await?;
    // parse node
    let mut item_node_ids: Vec<i64> = Vec::new();
    loop {
        let item = match item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(TransferItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        item_node_ids.push(id);
    }
    if item_node_ids.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(TransferItemError::IdConflictInGraphDBError);
    }
    if item_node_ids.is_empty() {
        // If visible_id does not exist
        return Err(TransferItemError::IdNotFoundInGraphDBError);
    }
    // drop item_node and item_nodes
    let _ = item_node;
    let _ = item_node_ids;

    //* validation of id is exist in GraphDB *//
    // get item node
    let mut new_parent_item_node = graphdb
        .execute(
            query("MATCH (item:Item {id: $id}) RETURN item")
                .param("id", transfer_item_data.new_parent_id),
        )
        .await?;
    // parse node
    let mut new_parent_item_node_ids: Vec<i64> = Vec::new();
    loop {
        let item = match new_parent_item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(TransferItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        new_parent_item_node_ids.push(id);
    }
    if new_parent_item_node_ids.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(TransferItemError::NewParentIdConflictInGraphDBError);
    }
    if new_parent_item_node_ids.is_empty() {
        // If visible_id does not exist
        return Err(TransferItemError::NewParentIdNotFoundInGraphDBError);
    }
    // drop item_node and item_nodes
    let _ = new_parent_item_node;
    let _ = new_parent_item_node_ids;

    //* validation of new_parent_id is not exist in visible_id's descendant items *//
    let mut descendant_item_nodes = graphdb
        .execute(
            query("MATCH path=(descendants)-[:Parent*]->(:Item {id: $id}) RETURN descendants")
                .param("id", transfer_item_data.id),
        )
        .await?;
    // parse nodes
    let mut descendant_item_node_ids: HashSet<i64> = HashSet::new();
    loop {
        let item = match descendant_item_nodes.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(TransferItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("descendants") {
            Ok(node) => node,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        descendant_item_node_ids.insert(id);
    }
    // add owm item id
    descendant_item_node_ids.insert(transfer_item_data.id as i64);
    // check new_parent_id is not descendant ids or own id.
    if descendant_item_node_ids.contains(&(transfer_item_data.new_parent_id as i64)) {
        // If new_parent_visible_id is descendant of visible_id
        return Err(TransferItemError::NewParentIdOneOfDescendantIdsError);
    }

    ////* operation *////
    //* get old_parent_id *//
    // get item node
    let mut old_parent_item_nodes = graphdb
        .execute(
            query("MATCH (:Item {id: $id})-[:Parent]->(parent) RETURN parent")
                .param("id", transfer_item_data.id),
        )
        .await?;
    // parse node
    let mut old_parent_item_node_ids: Vec<i64> = Vec::new();
    loop {
        let item = match old_parent_item_nodes.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(TransferItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("parent") {
            Ok(node) => node,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(TransferItemError::GraphDBDeError(e));
            }
        };
        old_parent_item_node_ids.push(id);
    }
    if old_parent_item_node_ids.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(TransferItemError::OldParentIdConflictInGraphDBError);
    }
    if old_parent_item_node_ids.is_empty() {
        // If visible_id does not exist
        return Err(TransferItemError::OldParentdNotFoundInGraphDBError);
    }
    // drop item_node and item_nodes
    let _ = old_parent_item_nodes;
    // get old_parent_id
    let old_parent_item_id = old_parent_item_node_ids[0];

    // transfer child_id from old_parent_id to new_parent_id
    match graphdb
            .run(
                query(
                    "MATCH (:Item {id: $child_id})-[relation:Parent]->(:Item {id: $old_parent_id}) DELETE relation WITH relation MATCH (child:Item {id: $child_id}) MATCH (new_parent:Item {id: $new_parent_id}) CREATE (child)-[:Parent]->(new_parent)"
                )
                .param("old_parent_id", old_parent_item_id)
                .param("new_parent_id",transfer_item_data.new_parent_id) 
                .param("child_id", transfer_item_data.id)
            )
            .await
        {
            Ok(_) => {
                tracing::info!("reconnect to new_parent_item_node in GraphDB");
            }
            Err(e) => {
                tracing::error!("Failed to reconnect item.");
                tracing::error!("Error: {:#?}", e);
                return Err(TransferItemError::GraphDBError(e));
            }
    }

    //* Discord Webhook *//
    let item_model = match Item::find_by_id(transfer_item_data.id).one(&rdb).await? {
        Some(label_model) => label_model,
        None => return Err(TransferItemError::IdNotFoundInItemTableError),
    };
    let sender = DiscordWebHookSender {
        title: "物品の移動情報".to_string(),
        description: "以下の物品が移動されました。".to_string(),
        color: 0x78e6d0,
        item: item_model.clone(),
        connect_discord_webhook: connect_discord_item_webhook,
    };
    discord_item_webhook_sender(sender).await?;

    Ok(())
}
