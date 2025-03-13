use crate::{
    handlers::joke::{teapot_handler, unavailable_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{routing::get, Router};

pub fn joke_route() -> Router<RwLockSharedState> {
    let joke_routes = Router::new()
        .route("/unavailable", get(unavailable_handler))
        .route("/teapot", get(teapot_handler));
    Router::new().nest("/joke", joke_routes)
}
