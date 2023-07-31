use dotenvy_macro::dotenv;
use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestUser {
    email: String,
    password: String,
    nickname: String,
}

impl TestUser {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        let nickname = Alphanumeric.sample_string(&mut rng, 8);
        let email = format!("testuser_{nickname}@mailinator.com");
        let password = Alphanumeric.sample_string(&mut rng, 16);

        Self {
            email,
            password,
            nickname,
        }
    }

    pub fn build_login_user(&self) -> LoginUser {
        LoginUser {
            email: &self.email,
            password: &self.password,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser<'a> {
    email: &'a str,
    password: &'a str,
}
