use dotenvy_macro::dotenv;
use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

const AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
const AUTH0_CONNECTION: &str = dotenv!("AUTH0_CONNECTION");
const AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");

pub async fn create_auth0_user(email: &str, nickname: &str, password: &str) -> Result<Auth0User> {
    let url = format!("https://{AUTH0_DOMAIN}/dbconnections/signup");
    let user_to_create = CreateAuth0User {
        client_id: AUTH0_CLIENT_ID.to_owned(),
        email: email.to_owned(),
        password: password.to_owned(),
        connection: AUTH0_CONNECTION.to_owned(),
        nickname: nickname.to_owned(),
    };
    let client = reqwest::Client::new();
    let created_user = client
        .post(url)
        .json(&user_to_create)
        .send()
        .await?
        .json::<Auth0User>()
        .await?;
    Ok(created_user)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth0User {
    #[serde(alias = "_id")]
    pub id: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateAuth0User {
    client_id: String,
    email: String,
    password: String,
    connection: String,
    nickname: String,
}
