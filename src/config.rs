use dotenvy_macro::dotenv;

#[derive(Clone)]
pub struct Config {
    pub address: [u8; 4],
    pub port: u16,
    pub auth0_domain: String,
}

impl Config {
    pub fn new() -> Self {
        let address = [0, 0, 0, 0];
        let port = 3000;
        let auth0_domain = dotenv!("AUTH0_DOMAIN").to_owned();

        Self {
            address,
            port,
            auth0_domain,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
