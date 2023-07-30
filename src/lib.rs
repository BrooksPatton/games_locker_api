mod api;
pub mod config;
pub mod db;
pub mod routes;

use config::Config;
use eyre::Result;
use routes::create_router;
use std::net::SocketAddr;

pub async fn run(config: Config) -> Result<()> {
    let app = create_router(config.clone());
    tracing_subscriber::fmt::init();
    let address = SocketAddr::from((config.address, config.port));
    tracing::info!("Server running on port {}", config.port);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
