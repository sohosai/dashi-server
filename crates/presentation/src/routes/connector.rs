use crate::{
    handlers::connector::{
        all_connectors_handler, register_handler, search_handler, status_handler,
    },
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn connector_route() -> Router<RwLockSharedState> {
    Router::new()
        .route("/connector", post(register_handler))
        .route("/connector", get(all_connectors_handler))
        .route("/connector/search", get(search_handler))
        .route("/connector/:id", patch(status_handler))
}
