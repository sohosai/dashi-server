use domain::{
    entity::data_type::individual_item::IndividualItemData,
    value_object::error::{critical_incident, item::individual::IndividualItemError},
};
use entity::{item::Entity as Item, label::Entity as Label};
use neo4rs::{query, Graph, Node};
use sea_orm::{DatabaseConnection, EntityTrait};

pub(super) async fn individual_item(
    rdb: DatabaseConnection,
    graphdb: Graph,
    id: u32,
) -> Result<IndividualItemData, IndividualItemError> {
    ////* operation and validation *////
    //* get individual item in Item Table *//
    let item_model = match Item::find_by_id(id as i32).all(&rdb).await {
        Ok(item_models) => {
            if item_models.is_empty() {
                return Err(IndividualItemError::IdNotFoundInItemTableError);
            }
            if item_models.len() > 1 {
                // If multiple visible_ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(IndividualItemError::IdConflictInItemTableError);
            }
            item_models[0].to_owned()
        }
        Err(e) => return Err(IndividualItemError::RDBError(e)),
    };

    //* get individual item in Label Table *//
    let label_model = match Label::find_by_id(item_model.visible_id.to_owned())
        .one(&rdb)
        .await?
    {
        Some(label_model) => label_model,
        None => return Err(IndividualItemError::LabelNotFoundError),
    };

    //* Root Item Process *//
    if item_model.id == 1 {
        let individual_item_data = IndividualItemData {
            id: item_model.id,
            visible_id: item_model.visible_id.to_owned(),
            parent_id: item_model.id,
            parent_visible_id: item_model.visible_id.to_owned(),
            record: label_model.record,
            name: item_model.name,
            product_number: item_model.product_number,
            description: item_model.description,
            purchase_year: item_model.purchase_year,
            purchase_price: item_model.purchase_price,
            durability: item_model.durability,
            is_depreciation: item_model.is_depreciation,
            connector: item_model.connector.to_owned(),
            is_rent: item_model.is_rent,
            color: item_model.color,
            created_at: item_model.created_at,
            updated_at: item_model.updated_at,
            recipient: item_model.recipient.to_owned(),
            rental_description: item_model.rental_description.to_owned(),
            latest_rent_at: item_model.latest_rent_at,
            scheduled_replace_at: item_model.scheduled_replace_at,
            latest_replace_at: item_model.latest_replace_at,
        };
        return Ok(individual_item_data);
    }

    //* get parent item node id in GraphDB *//
    let mut insert_graphdb_item_node = match graphdb
        .execute(
            query("MATCH (:Item {id: $id})-[:Parent]->(parent) RETURN parent")
                .param("id", item_model.id),
        )
        .await
    {
        Ok(graphdb) => graphdb,
        Err(e) => {
            return Err(IndividualItemError::GraphDBError(e));
        }
    };
    // parse node
    let mut ids: Vec<i64> = Vec::new();
    loop {
        let item = match insert_graphdb_item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                return Err(IndividualItemError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("parent") {
            Ok(node) => node,
            Err(e) => {
                return Err(IndividualItemError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                return Err(IndividualItemError::GraphDBDeError(e));
            }
        };
        ids.push(id);
    }
    // validation of parent item node id exists only one.
    if ids.len() > 1 {
        // If multiple visible_ids already exist
        //* critical incident *//
        critical_incident::conflict_error().await;
        return Err(IndividualItemError::IdConflictInGraphError);
    }
    // validation of parent item node id exists only one.
    if ids.is_empty() {
        return Err(IndividualItemError::IdNotFoundInGraphDBError);
    }
    //* get parent individual item in Item Table *//
    let parent_item_model = match Item::find_by_id(ids[0] as i32).all(&rdb).await {
        Ok(item_models) => {
            if item_models.is_empty() {
                return Err(IndividualItemError::ParentIdNotFoundInItemTableError);
            }
            if item_models.len() > 1 {
                // If multiple visible_ids already exist
                //* critical incident *//
                critical_incident::conflict_error().await;
                return Err(IndividualItemError::ParentIdConflictInItemTableError);
            }
            item_models[0].to_owned()
        }
        Err(e) => return Err(IndividualItemError::RDBError(e)),
    };

    //* validation of getting individual item in Label Table *//
    match Label::find_by_id(parent_item_model.visible_id.to_owned())
        .one(&rdb)
        .await?
    {
        Some(_) => {}
        None => return Err(IndividualItemError::ParentLabelNotFoundError),
    };

    //* convert Item and Label to IndividualItemData *//
    let individual_item_data = IndividualItemData {
        id: item_model.id,
        visible_id: item_model.visible_id.to_owned(),
        parent_id: parent_item_model.id,
        parent_visible_id: parent_item_model.visible_id.to_owned(),
        record: label_model.record,
        name: item_model.name,
        product_number: item_model.product_number,
        description: item_model.description,
        purchase_year: item_model.purchase_year,
        purchase_price: item_model.purchase_price,
        durability: item_model.durability,
        is_depreciation: item_model.is_depreciation,
        connector: item_model.connector.to_owned(),
        is_rent: item_model.is_rent,
        color: item_model.color,
        created_at: item_model.created_at,
        updated_at: item_model.updated_at,
        recipient: item_model.recipient.to_owned(),
        rental_description: item_model.rental_description.to_owned(),
        latest_rent_at: item_model.latest_rent_at,
        scheduled_replace_at: item_model.scheduled_replace_at,
        latest_replace_at: item_model.latest_replace_at,
    };

    Ok(individual_item_data)
}
