use crate::{entity::data_type::register_color::RegisterColorData, value_object::error::AppError};
use async_std::future::Future;

pub trait RegisterColorRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn register(
        &self,
        register_color_interface: RegisterColorInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct RegisterColorInterface {
    pub register_color_data: RegisterColorData,
}

impl RegisterColorInterface {
    pub async fn new(register_color_data: RegisterColorData) -> Self {
        Self {
            register_color_data,
        }
    }
}
