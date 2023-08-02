use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AccessToken {
    access_token: String,
    expires_in: String,
    token_type: String,
}
