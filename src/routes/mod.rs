pub mod healthcheck;
pub mod user;

use axum::{
    routing::{get, post},
    Router,
};
use eyre::Result;
use healthcheck::healthcheck;
use user::create_user;

pub async fn add_routes(router: Router) -> Result<Router> {
    let router = router
        .route("/healthcheck", get(healthcheck))
        .route("/create_user", post(create_user));
    Ok(router)
}
