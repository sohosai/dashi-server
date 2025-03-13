use domain::{
    entity::data_type::register_item::RegisterItemData,
    repository::{
        healthcheck::HealthCheckRepository,
        item::register::{RegisterItemInterface, RegisterItemRepository},
    },
    value_object::error::AppError,
};

pub struct RegisterItemInputs {
    pub register_item_data: RegisterItemData,
}

pub struct RegisterItemOutputs<T: HealthCheckRepository, S: RegisterItemRepository> {
    healyhcheck_repository: T,
    register_item_repository: S,
}

impl<T: HealthCheckRepository, S: RegisterItemRepository> RegisterItemOutputs<T, S> {
    pub async fn new(healyhcheck_repository: T, register_item_repository: S) -> Self {
        Self {
            healyhcheck_repository,
            register_item_repository,
        }
    }
    pub async fn run(&self, register_item_inputs: RegisterItemInputs) -> Result<(), AppError> {
        self.healyhcheck_repository.healthcheck().await?;
        let register_item_interface =
            RegisterItemInterface::new(register_item_inputs.register_item_data).await;
        self.register_item_repository
            .register(register_item_interface)
            .await
    }
}
