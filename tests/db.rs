use dotenvy_macro::dotenv;
use eyre::Result;
use sea_orm::{Database, DatabaseConnection};

const DB: &str = dotenv!("DB_URI");

pub async fn connect() -> Result<DatabaseConnection> {
    let database_connection = Database::connect(DB).await?;
    Ok(database_connection)
}
