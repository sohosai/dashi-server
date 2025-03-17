use crate::api::api;

mod api;
pub mod error;
pub mod handlers;
pub mod models;
pub mod multipart;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() {
    let _ = api().await;
}
