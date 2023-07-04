use dotenvy_macro::dotenv;
use eyre::Result;
use rand::prelude::*;
use reqwest::StatusCode;
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn healthcheck() -> Result<()> {
    let url = format!("{BASE_URL}/healthcheck");
    let request = reqwest::get(url).await?;
    let status_code = request.status();

    assert_eq!(status_code, 200);

    Ok(())
}

#[tokio::test]
async fn create_account() -> Result<()> {
    let mut rng = thread_rng();
    let nickname = format!("testuser{}", rng.gen::<u64>());
    let new_user = NewUser {
        email: format!("{nickname}@mailinator.com"),
        password: "paSSword0786(*&^)".to_owned(),
        nickname,
    };
    let url = format!("{BASE_URL}/create_user");
    let client = reqwest::Client::new();
    let request = client.post(url).json(&new_user).send().await?;
    let status = request.status();

    assert_eq!(status, StatusCode::CREATED);

    let db = connect().await?;

    db.close().await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUser {
    email: String,
    password: String,
    nickname: String,
}

const DB: &str = dotenv!("DB_URI");

pub async fn connect() -> Result<DatabaseConnection> {
    let database_connection = Database::connect(DB).await?;
    Ok(database_connection)
}
