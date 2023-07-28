pub mod users;

use axum::{routing::post, Router};
use users::create_user;

use crate::config::Config;

pub fn create_router(state: Config) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
