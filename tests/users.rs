mod config;
mod types;

use crate::types::user::{CreatedUser, TestUser};
use axum::http::StatusCode;
use config::Config;
use eyre::Result;

#[tokio::test]
async fn should_create_user() -> Result<()> {
    let config = Config::new();
    let new_user = TestUser::random();
    let client = reqwest::Client::new();
    let url = format!("{}/users", &config.base_url);
    let request = client.post(url).json(&new_user).send().await?;
    let status = request.status();

    assert_eq!(status, StatusCode::CREATED);

    let _created_user = request.json::<CreatedUser>().await?;

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
