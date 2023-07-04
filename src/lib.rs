pub mod api;
pub mod config;
pub mod routes;

use std::net::SocketAddr;

use config::Config;
use eyre::Result;
use routes::create_router;
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn run(config: &Config) -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        db: config.db.clone(),
    };

    let address = SocketAddr::from((config.address, config.port));

    tracing::debug!("listening on {}", address);

    let router = create_router(state).await?;

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
