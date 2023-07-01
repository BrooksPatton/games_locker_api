pub mod config;
pub mod routes;

use std::net::SocketAddr;

use axum::Router;
use config::Config;
use eyre::Result;
use routes::add_routes;

pub async fn run(config: &Config) -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = Router::new();

    let address = SocketAddr::from((config.address, config.port));

    tracing::debug!("listening on {}", address);

    let router = add_routes(router).await?;

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
