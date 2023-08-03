use serde::{Deserialize, Serialize};

use super::{game_tag::GameTag, optimal_session_length::OptimalSessionLength};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Game {
    id: Option<i32>,
    name: Option<i32>,
    players: Option<u8>,
    tag: Option<Vec<GameTag>>,
    optimal_session_length: Option<OptimalSessionLength>,
}
