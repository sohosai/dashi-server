use crate::error::csv::InsertCsvDataError;
use domain::entity::data_type::{meilisearch_color, meilisearch_connector};
use entity::{
    active_enum::Status,
    color::{self, Entity as Color},
    connector::{self, Entity as Connector},
};
use meilisearch_sdk::client::Client;
use regex::Regex;
use sea_orm::{self, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct ConnectorRecord {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ColorRecord {
    name: String,
    hex_color_code: String,
}

fn is_color_hex_color_code(input: &str) -> bool {
    let pattern = r"^#[a-z0-9]{6}$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(input)
}

fn is_color_name(input: &str) -> bool {
    let regex = Regex::new(r"^[a-z]+$").unwrap();
    regex.is_match(input)
}

fn is_connector_name(input: &str) -> bool {
    let regex = Regex::new(r"\^").unwrap();
    !regex.is_match(input)
}

pub async fn insert_csv_data(
    rdb: &DatabaseConnection,
    meilisearch: Client,
) -> Result<(), InsertCsvDataError> {
    //* Read csv files *//
    let mut connector_vec: Vec<ConnectorRecord> = Vec::new();
    let mut color_vec: Vec<ColorRecord> = Vec::new();
    let mut connector_reader = csv::Reader::from_path("./crates/init/csv/connector.csv")?;
    for result in connector_reader.deserialize() {
        let record: ConnectorRecord = result?;
        match is_connector_name(&record.name) {
            true => {
                connector_vec.push(record.clone());
                tracing::info!("{:?}", record);
            }
            false => {
                tracing::error!("Invalid connector name: {:?}", record.name);
                return Err(InsertCsvDataError::InvalidConnectorNameError);
            }
        }
    }
    let mut color_reader = csv::Reader::from_path("./crates/init/csv/color.csv")?;
    for result in color_reader.deserialize() {
        let record: ColorRecord = result?;
        match is_color_name(&record.name) {
            true => match is_color_hex_color_code(&record.hex_color_code) {
                true => {
                    color_vec.push(record.clone());
                    tracing::info!("{:?}", record);
                }
                false => {
                    tracing::error!("Invalid color hex color code: {:?}", record.hex_color_code);
                    return Err(InsertCsvDataError::InvalidColorHexColorCodeError);
                }
            },
            false => {
                tracing::error!("Invalid color name: {:?}", record.name);
                return Err(InsertCsvDataError::InvalidColorNameError);
            }
        }
    }

    //* Insert connectors and colors *//
    // Insert to Connector Table
    for record in connector_vec {
        let connector_model: connector::ActiveModel = connector::ActiveModel {
            name: Set(record.name),
            status: Set(Status::Active),
            ..Default::default()
        };
        match Connector::insert(connector_model).exec(rdb).await {
            Ok(connector_model) => {
                tracing::info!("Inserted to Connector Table: {:?}", connector_model);
            }
            Err(e) => {
                tracing::error!("Failed to insert connector.");
                return Err(InsertCsvDataError::DbErr(e));
            }
        }
    }
    // Get inserted connectors
    let connectors = Connector::find().all(rdb).await?;
    // Insert to Color Table
    for record in color_vec {
        let color_model: color::ActiveModel = color::ActiveModel {
            name: Set(record.name),
            hex_color_code: Set(record.hex_color_code),
            status: Set(Status::Active),
            ..Default::default()
        };
        match Color::insert(color_model).exec(rdb).await {
            Ok(color_model) => {
                tracing::info!("Inserted to Color Table: {:?}", color_model);
            }
            Err(e) => {
                tracing::error!("Failed to insert color.");
                return Err(InsertCsvDataError::DbErr(e));
            }
        }
    }
    // Get inserted colors
    let colors = Color::find().all(rdb).await?;

    // Insert to Meilisearch
    // Add connectors to meilisearch //
    let mut meilisearch_connector_models: Vec<meilisearch_connector::MeilisearchConnectorData> =
        Vec::new();
    for connector in connectors {
        let meilisearch_connector_model: meilisearch_connector::MeilisearchConnectorData =
            meilisearch_connector::MeilisearchConnectorData {
                id: connector.id,
                name: connector.name,
                status: connector.status,
            };
        meilisearch_connector_models.push(meilisearch_connector_model);
    }
    match meilisearch
        .index("connector")
        .add_documents(&meilisearch_connector_models, Some("id"))
        .await
    {
        Ok(insert_meilisearch_connector_model) => {
            tracing::info!("MeiliSearch result of connector.");
            tracing::info!("{:#?}", insert_meilisearch_connector_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert meilisearch.");
            return Err(InsertCsvDataError::MeiliSearchError(e));
        }
    }
    // set as filterable
    match meilisearch
        .index("connector")
        .set_filterable_attributes(["id", "name", "status"])
        .await
    {
        Ok(task) => {
            tracing::info!("Set filterable attributes result.");
            tracing::info!("{:#?}", task);
        }
        Err(e) => {
            tracing::error!("Failed to set filterable attributes.");
            return Err(InsertCsvDataError::MeiliSearchError(e));
        }
    }
    drop(meilisearch_connector_models);

    // Add colors to meilisearch //
    let mut meilisearch_color_models: Vec<meilisearch_color::MeilisearchColorData> = Vec::new();
    for color in colors {
        let meilisearch_color_model: meilisearch_color::MeilisearchColorData =
            meilisearch_color::MeilisearchColorData {
                id: color.id,
                name: color.name,
                hex_color_code: color.hex_color_code,
                status: color.status,
            };
        meilisearch_color_models.push(meilisearch_color_model);
    }
    match meilisearch
        .index("color")
        .add_documents(&meilisearch_color_models, Some("id"))
        .await
    {
        Ok(insert_meilisearch_color_model) => {
            tracing::info!("MeiliSearch result of color.");
            tracing::info!("{:#?}", insert_meilisearch_color_model);
        }
        Err(e) => {
            tracing::error!("Failed to insert meilisearch.");
            return Err(InsertCsvDataError::MeiliSearchError(e));
        }
    }
    // set as filterable
    match meilisearch
        .index("color")
        .set_filterable_attributes(["id", "name", "hex_color_code", "status"])
        .await
    {
        Ok(task) => {
            tracing::info!("Set filterable attributes result.");
            tracing::info!("{:#?}", task);
        }
        Err(e) => {
            tracing::error!("Failed to set filterable attributes.");
            return Err(InsertCsvDataError::MeiliSearchError(e));
        }
    }
    drop(meilisearch_color_models);

    Ok(())
}
