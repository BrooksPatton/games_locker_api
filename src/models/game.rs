use serde::{Deserialize, Serialize};

use super::game_tag::GameTag;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Game {
    id: Option<i32>,
    name: Option<i32>,
    players: Option<u8>,
    tag: Option<Vec<GameTag>>,
}
