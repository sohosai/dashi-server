use domain::{
    repository::{
        healthcheck::HealthCheckRepository,
        item::individual::{IndividualItemInterface, IndividualItemRepository},
    },
    value_object::error::AppError,
};
use entity::active_enum::Record;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct IndividualItemDataJson {
    pub id: i32,
    pub visible_id: String,
    pub parent_id: i32,
    pub parent_visible_id: String,
    pub record: Record,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Value,
    pub is_rent: bool,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
    pub recipient: String,
    pub rental_description: String,
    pub latest_rent_at: Option<String>,
    pub scheduled_replace_at: Option<String>,
    pub latest_replace_at: Option<String>,
}

pub struct IndividualItemInputs {
    pub id: u32,
}

pub struct IndividualItemOutputs<T: HealthCheckRepository, S: IndividualItemRepository> {
    healyhcheck_repository: T,
    individual_item_repository: S,
}

impl<T: HealthCheckRepository, S: IndividualItemRepository> IndividualItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, individual_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            individual_item_repository,
        }
    }
    pub async fn run(
        &self,
        inidividual_item_inputs: IndividualItemInputs,
    ) -> Result<IndividualItemDataJson, AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let individual_item_interface =
            IndividualItemInterface::new(inidividual_item_inputs.id).await;
        match self
            .individual_item_repository
            .individual(individual_item_interface)
            .await
        {
            Ok(result) => Ok(IndividualItemDataJson {
                id: result.id,
                visible_id: result.visible_id,
                parent_id: result.parent_id,
                parent_visible_id: result.parent_visible_id,
                record: result.record,
                name: result.name,
                product_number: result.product_number,
                description: result.description,
                purchase_year: result.purchase_year,
                purchase_price: result.purchase_price,
                durability: result.durability,
                is_depreciation: result.is_depreciation,
                connector: result.connector,
                is_rent: result.is_rent,
                color: result.color,
                created_at: result.created_at.to_string(),
                updated_at: result.updated_at.to_string(),
                recipient: result.recipient,
                rental_description: result.rental_description,
                latest_rent_at: result
                    .latest_rent_at
                    .map(|latest_rent_at| latest_rent_at.to_string()),
                scheduled_replace_at: result
                    .scheduled_replace_at
                    .map(|scheduled_replace_at| scheduled_replace_at.to_string()),
                latest_replace_at: result
                    .latest_replace_at
                    .map(|latest_replace_at| latest_replace_at.to_string()),
            }),
            Err(e) => Err(e),
        }
    }
}
