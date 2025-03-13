use domain::{
    repository::{
        healthcheck::HealthCheckRepository,
        item::delete::{DeleteItemInterface, DeleteItemRepository},
    },
    value_object::error::AppError,
};

pub struct DeleteItemInputs {
    pub id: u32,
}

pub struct DeleteItemOutputs<T: HealthCheckRepository, S: DeleteItemRepository> {
    healyhcheck_repository: T,
    delete_item_repository: S,
}

impl<T: HealthCheckRepository, S: DeleteItemRepository> DeleteItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, delete_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            delete_item_repository,
        }
    }
    pub async fn run(&self, delete_item_inputs: DeleteItemInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let delete_item_interface = DeleteItemInterface::new(delete_item_inputs.id).await;
        self.delete_item_repository
            .delete(delete_item_interface)
            .await
    }
}
