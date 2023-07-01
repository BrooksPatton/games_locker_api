pub mod healthcheck;

use axum::{routing::get, Router};
use eyre::Result;
use healthcheck::healthcheck;

pub async fn add_routes(router: Router) -> Result<Router> {
    let router = router.route("/healthcheck", get(healthcheck));
    Ok(router)
}
