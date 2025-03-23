use crate::{
    handlers::rental::{all_rental_items_handler, rent_handler, replace_handler, update_handler},
    models::rwlock_shared_state::RwLockSharedState,
};
use axum::{
    routing::{get, patch},
    Router,
};

pub fn rental_route() -> Router<RwLockSharedState> {
    let rental_routes = Router::new()
        .route("/all", get(all_rental_items_handler))
        .route("/rent/{id}", patch(rent_handler))
        .route("/update/{id}", patch(update_handler))
        .route("/replace/{id}", patch(replace_handler));
    Router::new().nest("/rental", rental_routes)
}
