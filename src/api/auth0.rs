use dotenvy_macro::dotenv;
use eyre::Result;

use crate::routes::users::types::CreateAuth0User;

pub async fn create_auth0_user(data: CreateAuth0UserData) -> Result<CreateAuth0UserResponse> {
    let url = format!("https://{}/dbconnections/signup", dotenv!("AUTH0_DOMAIN"));
    let client = reqwest::Client::new();
    Ok(client
        .post(url)
        .json(&data)
        .send()
        .await?
        .json::<CreateAuth0UserResponse>()
        .await?)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateAuth0UserResponse {
    pub _id: String,
    pub email_verified: bool,
    pub email: String,
    pub nickname: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateAuth0UserData {
    client_id: String,
    email: String,
    password: String,
    connection: String,
    nickname: String,
}

impl CreateAuth0UserData {
    pub fn new(email: String, password: String, nickname: String) -> Self {
        let client_id = dotenv!("AUTH0_CLIENT_ID").to_owned();
        let connection = dotenv!("AUTH0_CONNECTION").to_owned();

        Self {
            client_id,
            email,
            password,
            connection,
            nickname,
        }
    }
}

impl From<CreateAuth0User> for CreateAuth0UserData {
    fn from(value: CreateAuth0User) -> Self {
        Self::new(value.email, value.password, value.nickname)
    }
}
