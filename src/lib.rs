use std::net::SocketAddr;

use config::Config;
use eyre::Result;
use routes::create_router;

pub mod config;
mod routes;

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
