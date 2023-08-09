use serde::{Deserialize, Serialize};

use super::player::LoggedInAuth0Player;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

impl AccessToken {
    pub fn get_bearer_token(&self) -> String {
        format!("Bearer {}", &self.access_token)
    }
}

impl From<LoggedInAuth0Player> for AccessToken {
    fn from(value: LoggedInAuth0Player) -> Self {
        Self {
            access_token: value.access_token,
            expires_in: value.expires_in,
            token_type: value.token_type,
        }
    }
}
