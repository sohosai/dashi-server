use crate::{
    handlers::item::{
        delete_handler, image_handler, individual_item_handler, register_handler, search_handler,
        transfer_handler, trash_handler, update_handler,
    },
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn item_route() -> Router<RwLockSharedState> {
    let item_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/:id", get(individual_item_handler))
        .route("/archive/:limit", get(trash_handler))
        .route("/register", post(register_handler))
        .route("/update/:id", patch(update_handler))
        .route("/image/:id", put(image_handler))
        .route("/delete/:id", delete(delete_handler))
        .route("/trash", get(trash_handler))
        .route("/transfer", patch(transfer_handler));
    Router::new().nest("/item", item_routes)
}
