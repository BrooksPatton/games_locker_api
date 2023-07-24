mod config;
mod types;

use crate::types::new_user::CreateUser;
use axum::http::StatusCode;
use config::Config;
use eyre::Result;

#[tokio::test]
async fn should_create_user() -> Result<()> {
    let config = Config::new();
    let new_user = CreateUser::random();
    let client = reqwest::Client::new();
    let url = format!("{}/users", &config.base_url);
    let request = client.post(url).json(&new_user).send().await?;
    let status = request.status();

    assert_eq!(status, StatusCode::CREATED);
    Ok(())
}

#[tokio::test]
#[ignore = "tbd"]
async fn should_log_in() {}

#[tokio::test]
#[ignore = "tbd"]
async fn should_log_out() {}

#[tokio::test]
#[ignore = "tbd"]
async fn should_delete_user() {}

#[tokio::test]
#[ignore = "tbd"]
async fn should_access_protected_route() {}

#[tokio::test]
#[ignore = "tbd"]
async fn should_not_access_protected_route_without_token() {}
