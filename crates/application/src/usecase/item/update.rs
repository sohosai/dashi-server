use domain::{
    entity::data_type::update_item::UpdateItemData,
    repository::{
        healthcheck::HealthCheckRepository,
        item::update::{UpdateItemInterface, UpdateItemRepository},
    },
    value_object::error::AppError,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateItemDataJson {
    pub visible_id: String,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Vec<String>,
    pub color: String,
}

pub struct UpdateItemInputs {
    pub id: u32,
    pub update_item_data_json: UpdateItemDataJson,
}

pub struct UpdateItemOutputs<T: HealthCheckRepository, S: UpdateItemRepository> {
    healyhcheck_repository: T,
    update_item_repository: S,
}

impl<T: HealthCheckRepository, S: UpdateItemRepository> UpdateItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, update_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            update_item_repository,
        }
    }
    pub async fn run(&self, update_item_inputs: UpdateItemInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let update_item_data = UpdateItemData {
            id: update_item_inputs.id as i32,
            visible_id: update_item_inputs.update_item_data_json.visible_id.clone(),
            name: update_item_inputs.update_item_data_json.name.clone(),
            product_number: update_item_inputs
                .update_item_data_json
                .product_number
                .clone(),
            description: update_item_inputs.update_item_data_json.description.clone(),
            purchase_year: update_item_inputs.update_item_data_json.purchase_year,
            purchase_price: update_item_inputs.update_item_data_json.purchase_price,
            durability: update_item_inputs.update_item_data_json.durability,
            is_depreciation: update_item_inputs.update_item_data_json.is_depreciation,
            connector: update_item_inputs.update_item_data_json.connector.clone(),
            color: update_item_inputs.update_item_data_json.color.clone(),
        };
        let update_item_interface = UpdateItemInterface::new(update_item_data).await;
        self.update_item_repository
            .update(update_item_interface)
            .await
    }
}
