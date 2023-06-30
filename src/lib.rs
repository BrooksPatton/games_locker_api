pub mod config;

use std::net::SocketAddr;

use axum::Router;
use config::Config;
use eyre::Result;

pub async fn run(config: &Config) -> Result<()> {
    tracing_subscriber::fmt::init();

    let router = Router::new();

    let address = SocketAddr::from((config.address, config.port));

    tracing::debug!("listening on {}", address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
