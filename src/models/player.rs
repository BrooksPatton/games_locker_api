use super::{access_token::AccessToken, email::Email, player_games::PlayerGames};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
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
