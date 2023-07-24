use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuth0User {
    pub email: String,
    pub password: String,
    pub nickname: String,
}
