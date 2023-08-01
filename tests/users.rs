mod config;
mod types;

use crate::types::user::TestUser;
use axum::http::StatusCode;
use config::Config;
use eyre::Result;
use games_locker::types::user::User;

#[tokio::test]
async fn should_create_user() -> Result<()> {
    let config = Config::new();
    let new_user = TestUser::random();
    let client = reqwest::Client::new();
    let url = format!("{}/users", &config.base_url);
    let request = client.post(url).json(&new_user).send().await?;
    let status = request.status();

    assert_eq!(status, StatusCode::CREATED);

    let _created_user = request.json::<User>().await?;

    Ok(())
}

#[tokio::test]
async fn should_log_in() -> Result<()> {
    let config = Config::new();
    let new_user = TestUser::random();
    let url = format!("{}/users/login", &config.base_url);
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&new_user.build_login_user())
        .send()
        .await?;
    let status = response.status();

    assert_eq!(status, 200);

    let _user = response.json::<User>().await?;

    Ok(())
}

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
