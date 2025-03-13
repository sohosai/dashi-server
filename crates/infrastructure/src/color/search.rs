use crate::connection;
use domain::{
    entity::data_type::color::ColorData,
    repository::{
        color::search::{SearchColorInterface, SearchColorRepository},
        connection::ConnectionRepository,
    },
    value_object::error::AppError,
};
use search_module::search;
use serde::Deserialize;

pub mod search_module;

#[derive(Clone, Debug, Deserialize)]
pub struct SearchColor;

impl SearchColorRepository for SearchColor {
    async fn new() -> Self {
        Self {}
    }
    async fn search(
        &self,
        search_color_data: SearchColorInterface,
    ) -> Result<Vec<ColorData>, AppError> {
        let meilisearch = connection::CollectConnection::connect_meilisearch().await?;
        let result = search(meilisearch, search_color_data.keywords).await?;
        Ok(result)
    }
}
