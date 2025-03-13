use crate::{entity::data_type::color::ColorData, value_object::error::AppError};
use async_std::future::Future;

pub trait SearchColorRepository: Send + Sync + 'static {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn search(
        &self,
        search_color_interface: SearchColorInterface,
    ) -> impl Future<Output = Result<Vec<ColorData>, AppError>> + Send;
}

pub struct SearchColorInterface {
    pub keywords: String,
}

impl SearchColorInterface {
    pub async fn new(keywords: String) -> Self {
        Self { keywords }
    }
}
