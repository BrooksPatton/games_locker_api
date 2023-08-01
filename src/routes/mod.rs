pub mod users;

use axum::{routing::post, Router};
use users::{create_user, login};

use crate::config::Config;

pub fn create_router(state: Config) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
