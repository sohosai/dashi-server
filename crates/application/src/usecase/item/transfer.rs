use domain::{
    entity::data_type::transfer_item::TransferItemData,
    repository::{
        healthcheck::HealthCheckRepository,
        item::transfer::{TransferItemInterface, TransferItemRepository},
    },
    value_object::error::AppError,
};

pub struct TransferItemInputs {
    pub transfer_item_data: TransferItemData,
}

pub struct TransferItemOutputs<T: HealthCheckRepository, S: TransferItemRepository> {
    healyhcheck_repository: T,
    transfer_item_repository: S,
}

impl<T: HealthCheckRepository, S: TransferItemRepository> TransferItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, transfer_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            transfer_item_repository,
        }
    }
    pub async fn run(&self, transfer_item_inputs: TransferItemInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let transfer_item_interface =
            TransferItemInterface::new(transfer_item_inputs.transfer_item_data).await;
        self.transfer_item_repository
            .transfer(transfer_item_interface)
            .await
    }
}
