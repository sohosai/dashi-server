use crate::connection;
use all_colors_module::all_colors;
use domain::{
    entity::data_type::color::ColorData,
    repository::{color::all_colors::AllColorsRepository, connection::ConnectionRepository},
    value_object::error::AppError,
};
use serde::Deserialize;

pub mod all_colors_module;

#[derive(Clone, Debug, Deserialize)]
pub struct AllColors;

impl AllColorsRepository for AllColors {
    async fn new() -> Self {
        Self {}
    }
    async fn all_colors(&self) -> Result<Vec<ColorData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = all_colors(rdb).await?;
        Ok(result)
    }
}
