mod api;
pub mod db;
pub mod models;
pub mod router;

use dotenvy_macro::dotenv;
use eyre::Result;
use router::create_router;
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;

use crate::models::app_state::AppState;
pub struct Server {
    address: [u8; 4],
    port: u16,
    db: DatabaseConnection,
}

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

impl Server {
    pub async fn new() -> Result<Self> {
        let db = Database::connect(DATABASE_URL).await?;
        Ok(Self {
            address: [127, 0, 0, 1],
            port: 3000,
            db,
        })
    }

    pub async fn serve(&self) -> Result<()> {
        let app_state = AppState::new(self.db.clone());
        let router = create_router(app_state);
        tracing_subscriber::fmt::init();
        let address = SocketAddr::from((self.address, self.port));

        tracing::info!("Server listening on port {}", self.port);
        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;
        Ok(())
    }
}
