use crate::value_object::error::healthcheck::HealthCheckError;
use async_std::future::Future;

pub trait HealthCheckRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn healthcheck(&self) -> impl Future<Output = Result<(), HealthCheckError>> + Send;
}
