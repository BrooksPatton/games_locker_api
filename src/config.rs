use dotenvy_macro::dotenv;
use eyre::Result;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Config {
    pub address: [u8; 4],
    pub port: u16,
    pub auth0_domain: String,
    pub db: DatabaseConnection,
}

impl Config {
    pub async fn new() -> Result<Self> {
        let address = [0, 0, 0, 0];
        let port = 3000;
        let auth0_domain = dotenv!("AUTH0_DOMAIN").to_owned();
        let db = sea_orm::Database::connect(dotenv!("DATABASE_URL")).await?;

        Ok(Self {
            address,
            port,
            auth0_domain,
            db,
        })
    }
}
