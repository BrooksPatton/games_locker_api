use dotenvy_macro::dotenv;
use entity::users::Model;
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};

use super::auth0_token::Auth0Token;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct User {
    id: Option<i32>,
    email: Option<String>,
    password: Option<String>,
    nickname: Option<String>,
    _id: Option<String>,
    email_verified: Option<bool>,
    pub auth0_token: Option<Auth0Token>,
}

impl User {
    pub fn build_create_auth0_user(&self) -> Option<CreateAuth0User> {
        Some(CreateAuth0User {
            email: self.email.clone()?,
            password: self.password.clone()?,
            nickname: self.nickname.clone()?,
            client_id: dotenv!("AUTH0_CLIENT_ID").to_owned(),
            connection: dotenv!("AUTH0_CONNECTION").to_owned(),
        })
    }

    pub fn get_auth0_id(&self) -> Result<&str> {
        let Some(id) = &self._id else {
            bail!("Missing Auth0 Id");
        };

        Ok(id)
    }

    pub fn get_email(&self) -> Result<&str> {
        let Some(email) = &self.email else {
            bail!("Missing email");
        };

        Ok(email)
    }

    pub fn get_nickname(&self) -> Result<&str> {
        let Some(nickname) = &self.nickname else {
            bail!("Missing nickname");
        };

        Ok(nickname)
    }

    pub fn build_created_user(&self) -> Result<CreatedUser> {
        let Some(id) = &self.id else { bail!("Missing id when building created user") };
        let Some(email) = &self.email else { bail!("Missing email when building created user")};
        let Some(nickname) = &self.nickname else { bail!("Missing nickname when building created user")};

        Ok(CreatedUser {
            id: *id,
            email: email.to_owned(),
            nickname: nickname.to_owned(),
        })
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            id: Some(value.id),
            email: Some(value.email),
            nickname: Some(value.nickname),
            _id: Some(value.auth0_id),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateAuth0User {
    email: String,
    password: String,
    nickname: String,
    client_id: String,
    connection: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CreatedUser {
    pub id: i32,
    pub email: String,
    pub nickname: String,
}
