use crate::{
    handlers::item::{
        archive_handler, delete_handler, individual_item_handler, register_handler, search_handler,
        transfer_handler, update_handler,
    },
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub fn item_route() -> Router<RwLockSharedState> {
    let item_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/:id", get(individual_item_handler))
        .route("/archive/:limit", get(archive_handler))
        .route("/register", post(register_handler))
        .route("/update/:id", patch(update_handler))
        .route("/delete/:id", delete(delete_handler))
        .route("/transfer", patch(transfer_handler));
    Router::new().nest("/item", item_routes)
}
