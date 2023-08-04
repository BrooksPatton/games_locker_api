pub mod models;
pub mod router;

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
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
