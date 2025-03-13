use crate::connection;
use domain::{
    repository::{
        color::update::{UpdateColorInterface, UpdateColorRepository},
        connection::ConnectionRepository,
    },
    value_object::error::AppError,
};
use serde::Deserialize;
use update_module::update;

pub mod update_module;

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateColor;

impl UpdateColorRepository for UpdateColor {
    async fn new() -> Self {
        Self {}
    }
    async fn update(&self, update_color_data: UpdateColorInterface) -> Result<(), AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let rdb = connection::CollectConnection::connect_rdb().await?;
        update(rdb, meilisearch, update_color_data.update_color_data).await?;
        Ok(())
    }
}
