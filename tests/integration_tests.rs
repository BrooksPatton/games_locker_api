use eyre::Result;
use games_locker::models::{email::Email, player::Player};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn test_authentication() -> Result<()> {
    // create account
    let url = format!("{BASE_URL}/players");
    let mut rng = thread_rng();
    let nickname = Alphanumeric.sample_string(&mut rng, 16);
    let password = Alphanumeric.sample_string(&mut rng, 16);
    let email = format!("{nickname}@mailinator.com");
    let player = Player {
        email: Some(Email(email.clone())),
        nickname: Some(nickname),
        password: Some(password.clone()),
        ..Default::default()
    };
    let client = reqwest::Client::new();
    let response = client.post(url).json(&player).send().await?;
    let status = response.status();
    let expected_status = 201;

    assert_eq!(status, expected_status);
    // login
    let url = format!("{BASE_URL}/players/login");
    let player = Player {
        email: Some(Email(email)),
        password: Some(password),
        ..Default::default()
    };
    let response = client.post(url).json(&player).send().await?;
    let status = response.status();
    let expected_status = 200;

    assert_eq!(status, expected_status);

    let player = response.json::<Player>().await?;

    // access secure route with token
    let url = format!("{BASE_URL}/healthcheck/secure");
    let token = player
        .auth0_access_token
        .expect("missing token")
        .get_bearer_token();
    Ok(())
}
