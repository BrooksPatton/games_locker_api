use eyre::Result;
use sea_orm::DatabaseConnection;

pub struct Config {
    pub port: u16,
    pub address: [u8; 4],
    pub db: DatabaseConnection,
}

impl Config {
    pub fn new(port: &str, db: DatabaseConnection) -> Result<Self> {
        let port = port.parse()?;
        let address = [0, 0, 0, 0];

        Ok(Self { port, address, db })
    }

    pub fn set_address(&mut self, address: [u8; 4]) -> &mut Self {
        self.address = address;
        self
    }
}
