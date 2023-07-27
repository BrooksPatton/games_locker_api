use eyre::Result;
use sea_orm::DatabaseConnection;

pub async fn insert_user(
    db: &DatabaseConnection,
    auth0_id: &str,
    email: &str,
    nickname: &str,
) -> Result<()> {
}
