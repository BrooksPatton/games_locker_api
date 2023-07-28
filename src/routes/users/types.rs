use entity::users::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuth0User {
    pub email: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub auth0_id: String,
    pub email: String,
    pub nickname: String,
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            auth0_id: value.auth0_id,
            email: value.email,
            nickname: value.nickname,
        }
    }
}
