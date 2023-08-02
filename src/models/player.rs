use super::{access_token::AccessToken, email::Email, player_games::PlayerGames};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Player {
    auth0_access_token: Option<AccessToken>,
    auth0_id: Option<String>,
    email: Option<Email>,
    email_verified: Option<bool>,
    epic_access_token: Option<AccessToken>,
    family_name: Option<String>,
    games: Option<PlayerGames>,
    given_name: Option<String>,
    id: Option<i32>,
    name: Option<String>,
    nickname: Option<String>,
    picture: Option<String>,
    steam_access_token: Option<AccessToken>,
    username: Option<String>,
}
