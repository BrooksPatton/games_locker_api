use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Users {
    id: Option<i32>,
    auth0_id: Option<String>,
    email_verified: Option<bool>,
    email: Option<Email>,
    username: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    name: Option<String>,
    nickname: Option<String>,
    picture: Option<String>,
    access_token: Option<AccessToken>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Email(String);

#[derive(Serialize, Deserialize, Default, Clone)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: String,
}
