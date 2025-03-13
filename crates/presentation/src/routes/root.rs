use crate::{
    models::rwlock_shared_state::RwLockSharedState,
    routes::{
        csv::csv_route, item::item_route, joke::joke_route, rental::rent_route, util::util_route,
    },
};
use axum::Router;

use super::{color::color_route, connector::connector_route};

pub fn root_route() -> Router<RwLockSharedState> {
    let root_routes = Router::new()
        .merge(color_route())
        .merge(connector_route())
        .merge(csv_route())
        .merge(item_route())
        .merge(rent_route())
        .merge(util_route())
        .merge(joke_route());
    Router::new().nest("/api", root_routes)
}
