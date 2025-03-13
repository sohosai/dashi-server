use crate::error::initialize::InitializeError;
use domain::{entity::data_type::meilisearch_item, repository::connection::ConnectionRepository};
use entity::{
    active_enum::Record,
    item::{self, Entity as Item},
    label::{self, Entity as Label},
};
use infrastructure::connection;
use neo4rs::{query, Node};
use sea_orm::{self, EntityTrait, Set};

pub(super) async fn initialize() -> Result<(), InitializeError> {
    // Connect rdb
    let rdb = match connection::CollectConnection::connect_rdb().await {
        Ok(rdb) => rdb,
        Err(e) => {
            tracing::error!("Failed to connect to PostgreSQL.");
            return Err(InitializeError::ConnectionError(e));
        }
    };
    // Connect graphdb
    let graphdb = match connection::CollectConnection::connect_graphdb().await {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to connect to Neo4j.");
            return Err(InitializeError::ConnectionError(e));
        }
    };
    // Connect meilisearch
    let meilisearch = match connection::CollectConnection::connect_meilisearch().await {
        Ok(meilisearch) => meilisearch,
        Err(e) => {
            tracing::error!("Failed to connect to Meilisearch.");
            return Err(InitializeError::ConnectionError(e));
        }
    };
    // Connect r2
    let r2 = match connection::CollectConnection::connect_object_strage().await {
        Ok(r2) => r2,
        Err(e) => {
            tracing::error!("Failed to connect to R2.");
            return Err(InitializeError::ConnectionError(e));
        }
    };

    // Add rdb data //
    // Insert data into the Label table
    let label_model: label::ActiveModel = label::ActiveModel {
        visible_id: Set("0000".to_string()),
        is_max: Set(true),
        record: Set(Record::Nothing),
    };
    let inserted_label_model = Label::insert(label_model).exec(&rdb).await;
    match inserted_label_model {
        Ok(label_model) => {
            tracing::info!("Inserted to Label Table: {:?}", label_model);
        }
        Err(e) => {
            let a: sea_orm::DbErr = e;
            tracing::error!("Failed to insert label.");
            return Err(InitializeError::DbErr(a));
        }
    }
    // Insert data into the Item table
    let root_item_connector: Vec<String> = Vec::new();
    let item_model: item::ActiveModel = item::ActiveModel {
        visible_id: Set("0000".to_string()),
        name: Set("筑波大学".to_string()),
        product_number: Set("".to_string()),
        description: Set("根の物品です。".to_string()),
        is_depreciation: Set(false),
        connector: Set(serde_json::json!(root_item_connector)),
        is_rent: Set(false),
        color: Set("".to_string()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    match Item::insert(item_model).exec(&rdb).await {
        Ok(item_model) => {
            tracing::info!("Inserted to Item Table: {:?}", item_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert item.");
            return Err(InitializeError::DbErr(e));
        }
    };

    // Add meilisearch data //
    let meilisearch_model: meilisearch_item::MeilisearchItemData =
        meilisearch_item::MeilisearchItemData {
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
            connector: root_item_connector,
            is_rent: false,
            color: "".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
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
            return Err(InitializeError::MeiliSearchError(e));
        }
    }
    // set as filterable
    match meilisearch
        .index("item")
        .set_filterable_attributes([
            "id",
            "visible_id",
            "record",
            "name",
            "product_number",
            "description",
            "purchase_year",
            "purchase_price",
            "durability",
            "is_depreciation",
            "connector",
            "is_rent",
            "color",
            "created_at",
            "updated_at",
            "recipient",
            "rental_description",
            "latest_rent_at",
            "scheduled_replace_at",
            "latest_replace_at",
        ])
        .await
    {
        Ok(task) => {
            tracing::info!("Set filterable attributes result.");
            tracing::info!("{:#?}", task);
        }
        Err(e) => {
            tracing::error!("Failed to set filterable attributes.");
            return Err(InitializeError::MeiliSearchError(e));
        }
    }

    // Add graphdb data //
    match graphdb
        .to_owned()
        .run(query("CREATE (item:Item {id: $id})").param("id", 1))
        .await
    {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to create item node.");
            return Err(InitializeError::GraphDBError(e));
        }
    };
    // get node
    let mut insert_graphdb_item_node = match graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
    {
        Ok(graphdb) => graphdb,
        Err(e) => {
            tracing::error!("Failed to get item node.");
            return Err(InitializeError::GraphDBError(e));
        }
    };
    // parse node
    loop {
        let item = match insert_graphdb_item_node.next().await {
            Ok(item) => item,
            Err(e) => {
                tracing::error!("Failed to get item.");
                return Err(InitializeError::GraphDBError(e));
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                tracing::error!("Failed to get node.");
                return Err(InitializeError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to get id.");
                return Err(InitializeError::GraphDBDeError(e));
            }
        };
        tracing::info!("GraphDB result of item.");
        tracing::info!("node: {:#?}", id);
    }

    // Add r2 data //
    match r2
        .upload_file(
            "1.webp",
            "image/webp",
            "./crates/init/image/tsukuba.webp",
            None,
        )
        .await
    {
        Ok(_) => {
            tracing::info!("Uploaded image file.");
        }
        Err(e) => {
            tracing::error!("Failed to upload file.");
            return Err(InitializeError::CfR2SdkOperationError(e));
        }
    };

    // add connectors and colors
    crate::csv::insert_csv_data(&rdb, meilisearch).await?;

    // Close rdb
    let _ = rdb.close().await;

    // Finish!
    tracing::info!("Initialize was finished!");
    Ok(())
}
