use crate::{entity::data_type::update_color::UpdateColorData, value_object::error::AppError};
use async_std::future::Future;

pub trait UpdateColorRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn update(
        &self,
        update_color_interface: UpdateColorInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct UpdateColorInterface {
    pub update_color_data: UpdateColorData,
}

impl UpdateColorInterface {
    pub async fn new(update_color_data: UpdateColorData) -> Self {
        Self { update_color_data }
    }
}
