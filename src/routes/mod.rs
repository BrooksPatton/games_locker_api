pub mod healthcheck;
pub mod user;

use axum::{
    routing::{get, post},
    Router,
};
use eyre::Result;
use healthcheck::healthcheck;
use user::create_user;

use crate::AppState;

pub async fn create_router(state: AppState) -> Result<Router> {
    let router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/create_user", post(create_user))
        .with_state(state);
    Ok(router)
}
