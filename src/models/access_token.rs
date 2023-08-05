use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AccessToken {
    access_token: String,
    expires_in: String,
    token_type: String,
}

impl AccessToken {
    pub fn get_bearer_token(&self) -> String {
        format!("Bearer {}", &self.access_token)
    }
}
