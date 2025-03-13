use crate::{
    handlers::rental::{rent_handler, replace_handler, update_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{routing::patch, Router};

pub fn rent_route() -> Router<RwLockSharedState> {
    let rent_routes = Router::new()
        .route("/rent/:id", patch(rent_handler))
        .route("/update/:id", patch(update_handler))
        .route("/replace/:id", patch(replace_handler));
    Router::new().nest("/rental", rent_routes)
}
