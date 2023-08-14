use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

pub struct Player {
    pub db_id: Option<i32>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub auth0_id: Option<String>,
}

#[cfg(test)]
impl Player {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let nickname = Alphanumeric.sample_string(&mut rng, 16);
        let email = format!("{&nickname}@mailinator.com");
        let password = Alphanumeric.sample_string(&mut rng, 16);

        Self {
            db_id: None,
            nickname,
            email,
            password,
            auth0_id: None,
        }
    }
}
