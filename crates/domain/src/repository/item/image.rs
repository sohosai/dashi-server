use crate::{entity::data_type::image_item::ImageItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait ImageItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn image(
        &self,
        image_item_interface: ImageItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct ImageItemInterface {
    pub image_item_data: ImageItemData,
}

impl ImageItemInterface {
    pub async fn new(image_item_data: ImageItemData) -> Self {
        Self { image_item_data }
    }
}
