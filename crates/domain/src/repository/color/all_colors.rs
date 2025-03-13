use crate::{entity::data_type::color::ColorData, value_object::error::AppError};
use async_std::future::Future;

pub trait AllColorsRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn all_colors(&self) -> impl Future<Output = Result<Vec<ColorData>, AppError>> + Send;
}
