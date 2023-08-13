use super::games::Game;

pub struct Player {
    pub id: Option<i32>,
    pub stream_key: Option<String>,
    pub games: Option<Vec<Game>>,
}
