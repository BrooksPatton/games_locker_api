use super::{game::Game, game_state::GameState, player::Player};

pub struct PlayerGame {
    pub id: i32,
    pub state: GameState,
    pub goal: String,
    pub completed: bool,
    pub player: Player,
    pub game: Game,
}
