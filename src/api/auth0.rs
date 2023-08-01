use crate::types::user::User;
use dotenvy_macro::dotenv;
use eyre::{eyre, Result};

pub async fn create_auth0_user(data: User) -> Result<User> {
    let url = format!("https://{}/dbconnections/signup", dotenv!("AUTH0_DOMAIN"));
    let client = reqwest::Client::new();
    let user = client
        .post(url)
        .json(
            &data
                .build_create_auth0_user()
                .ok_or_else(|| eyre!("Missing data to create auth0 user"))?,
        )
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user)
}

pub async fn login(data: User) -> Result<User> {}
