use crate::{
    handlers::csv::{depreiation_handler, item_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{routing::get, Router};

pub fn csv_route() -> Router<RwLockSharedState> {
    let csv_routes = Router::new()
        .route("/depreiation", get(depreiation_handler))
        .route("/item", get(item_handler));
    Router::new().nest("/csv", csv_routes)
}
