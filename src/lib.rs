pub mod models;
pub mod router;

use std::net::SocketAddr;

use eyre::Result;
use router::create_router;
pub struct Server {
    address: [u8; 4],
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        Self {
            address: [127, 0, 0, 1],
            port: 3000,
        }
    }

    pub async fn serve(&self) -> Result<()> {
        let router = create_router();
        tracing_subscriber::fmt::init();
        let address = SocketAddr::from((self.address, self.port));

        tracing::info!("Server listening on port {}", self.port);
        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
