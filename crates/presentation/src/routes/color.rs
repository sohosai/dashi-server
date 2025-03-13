use crate::{
    handlers::color::{all_colors_handler, register_handler, search_handler, update_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn color_route() -> Router<RwLockSharedState> {
    Router::new()
        .route("/color", post(register_handler))
        .route("/color", get(all_colors_handler))
        .route("/color/search", get(search_handler))
        .route("/color/:id", patch(update_handler))
}
