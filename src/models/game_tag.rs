use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum GameTag {
    #[default]
    None,
}
