use crate::models::player::{Auth0PlayerLogin, Auth0SignupPlayer, Player, PlayerError};
use dotenvy_macro::dotenv;
use eyre::{bail, Result};

const AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");

pub async fn signup(player: &Auth0SignupPlayer) -> Result<()> {
    let url = format!("https://{AUTH0_DOMAIN}/dbconnections/signup");
    let client = reqwest::Client::new();
    let response = client.post(url).json(player).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(())
    } else {
        Err(PlayerError::Auth0Signup.into())
    }
}

pub async fn login(player: &Auth0PlayerLogin) -> Result<Player> {
    let url = format!("https://{AUTH0_DOMAIN}/oauth/token");
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(player)
        .send()
        .await
        .map_err(|error| {
            tracing::error!("Error sending login request to Auth0: {error}");
            eyre::eyre!("There was a problem logging in, please try again later")
        })?;

    if !response.status().is_success() {
        bail!("There was a problem logging in, please try again");
    }

    let player = response.json().await.map_err(|error| {
        tracing::error!("Error converting responce to player: {error}");
        eyre::eyre!("There was a problem logging in, please try again later")
    })?;

    Ok(player)
}
