use crate::{
    handlers::utils::{generate_handler, healthcheck_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn util_route() -> Router<RwLockSharedState> {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/generate", post(generate_handler))
}
