use super::{access_token::AccessToken, email::Email, player_games::PlayerGames};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Player {
    auth0_access_token: Option<AccessToken>,
    auth0_id: Option<String>,
    email: Option<Email>,
    epic_access_token: Option<AccessToken>,
    games: Option<PlayerGames>,
    id: Option<i32>,
    nickname: Option<String>,
    steam_access_token: Option<AccessToken>,
}
