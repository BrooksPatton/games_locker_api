use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PlayerGames {
    finished: Option<bool>,
    game_id: Option<i32>,
    goal: Option<bool>,
    ignoring: Option<bool>,
    player_id: Option<i32>,
    playing: Option<bool>,
    replay: Option<bool>,
}
