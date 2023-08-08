use super::{access_token::AccessToken, email::Email, player_games::PlayerGames};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const AUTH0_CLIENT_ID: &str = dotenv!("AUTH0_CLIENT_ID");
const AUTH0_CONNECTION: &str = dotenv!("AUTH0_CONNECTION");
const AUTH0_SECRET: &str = dotenv!("AUTH0_SECRET");
const AUTH0_DOMAIN: &str = dotenv!("AUTH0_DOMAIN");

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Player {
    pub auth0_access_token: Option<AccessToken>,
    pub auth0_id: Option<String>,
    pub email: Option<Email>,
    pub epic_access_token: Option<AccessToken>,
    pub games: Option<PlayerGames>,
    pub id: Option<i32>,
    pub nickname: Option<String>,
    pub steam_access_token: Option<AccessToken>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Auth0SignupPlayer {
    client_id: String,
    email: String,
    password: String,
    connection: String,
    nickname: String,
}

impl TryFrom<Player> for Auth0SignupPlayer {
    type Error = PlayerError<'static>;

    fn try_from(value: Player) -> Result<Self, Self::Error> {
        let client_id = AUTH0_CLIENT_ID.to_owned();
        let Some(email) = value.email else {
            return Err(PlayerError::MissingPlayerValue("email"));
        };
        let Some(password) = value.password else {
            return Err(PlayerError::MissingPlayerValue("password"));
        };
        let connection = AUTH0_CONNECTION.to_owned();
        let Some(nickname) = value.nickname else {
            return Err(PlayerError::MissingPlayerValue("nickname"));
        };

        Ok(Self {
            client_id,
            email: (*email).to_owned(),
            password,
            connection,
            nickname,
        })
    }
}

#[derive(Error, Debug)]
pub enum PlayerError<'a> {
    #[error("Error: missing {0}")]
    MissingPlayerValue(&'a str),
    #[error("Error signing up player in Auth0")]
    Auth0Signup,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Auth0PlayerLogin {
    grant_type: String,
    client_id: String,
    client_secret: String,
    audience: String,
    username: String,
    password: String,
    scope: String,
    realm: String,
}

impl TryFrom<Player> for Auth0PlayerLogin {
    type Error = PlayerError<'static>;

    fn try_from(value: Player) -> Result<Self, Self::Error> {
        let grant_type = "http://auth0.com/oauth/grant-type/password-realm".to_owned();
        let client_id = AUTH0_CLIENT_ID.to_owned();
        let client_secret = AUTH0_SECRET.to_owned();
        let audience = format!("https://{AUTH0_DOMAIN}/api/v2/").to_owned();
        let Some(username) = value.email else {
            return Err(PlayerError::MissingPlayerValue("email"));
        };
        let Some(password) = value.password else {
            return Err(PlayerError::MissingPlayerValue("password"));
        };
        let scope = "openid profile email".to_owned();
        let realm = AUTH0_CONNECTION.to_owned();

        Ok(Self {
            grant_type,
            client_id,
            client_secret,
            audience,
            username: (*username).clone(),
            password,
            scope,
            realm,
        })
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerResponse {
    auth0_access_token: String,
    email: String,
    games: Vec<PlayerGames>,
    id: i32,
    nickname: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UpsertPlayer {
    pub auth0_id: String,
    pub email: Email,
    pub id: Option<i32>,
    pub nickname: String,
}

impl TryFrom<Player> for UpsertPlayer {
    type Error = PlayerError<'static>;

    fn try_from(value: Player) -> Result<Self, Self::Error> {
        let Some(auth0_id) = value.auth0_id else {
            return Err(PlayerError::MissingPlayerValue("auth0 id"));
        };
        let Some(email) = value.email else {
            return Err(PlayerError::MissingPlayerValue("email"));
        };
        let Some(nickname) = value.nickname else {
            return Err(PlayerError::MissingPlayerValue("nickname"));
        };

        Ok(Self {
            auth0_id,
            email,
            id: value.id,
            nickname,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoggedInAuth0Player {
    access_token: String,
    id_token: String,
    scope: String,
    expires_in: u32,
    token_type: String,
}

impl From<LoggedInAuth0Player> for Player {
    fn from(value: LoggedInAuth0Player) -> Self {
        Self {
            auth0_access_token: todo!(),
            auth0_id: todo!(),
            email: todo!(),
            epic_access_token: todo!(),
            games: todo!(),
            id: todo!(),
            nickname: todo!(),
            steam_access_token: todo!(),
            password: todo!(),
        }
    }
}
