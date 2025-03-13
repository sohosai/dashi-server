use domain::{
    repository::{
        color::register::{RegisterColorInterface, RegisterColorRepository},
        connection::ConnectionRepository,
    },
    value_object::error::AppError,
};
use register_module::register;
use serde::Deserialize;

use crate::connection;

pub mod register_module;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterColor;

impl RegisterColorRepository for RegisterColor {
    async fn new() -> Self {
        Self {}
    }
    async fn register(&self, register_color_data: RegisterColorInterface) -> Result<(), AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let rdb = connection::CollectConnection::connect_rdb().await?;
        register(rdb, meilisearch, register_color_data.register_color_data).await?;
        Ok(())
    }
}
