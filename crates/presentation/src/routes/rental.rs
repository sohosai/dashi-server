use crate::{
    handlers::rental::{render_handler, rent_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{post, put},
    Router,
};

pub fn rent_route() -> Router<RwLockSharedState> {
    let rent_routes = Router::new()
        .route("/rent", post(rent_handler))
        .route("/return/:visible_id", put(render_handler));
    Router::new().nest("/rent", rent_routes)
}
