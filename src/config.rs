use eyre::Result;

pub struct Config {
    pub port: u16,
    pub address: [u8; 4],
}

impl Config {
    pub fn new(port: &str) -> Result<Self> {
        let port = port.parse()?;
        let address = [0, 0, 0, 0];

        Ok(Self { port, address })
    }

    pub fn set_address(&mut self, address: [u8; 4]) -> &mut Self {
        self.address = address;
        self
    }
}
