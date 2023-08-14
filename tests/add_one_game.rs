mod config;

use config::TestConfig;
use eyre::Result;
use games_locker::models::player::Player;

#[cfg(test)]
#[tokio::test]
async fn add_one_game() -> Result<()> {
    let config = TestConfig::new();
    // create account
    let new_player = Player::new_random();
    // log in
    // create game
    // get game
    // verify game has
    //   expected tag
    //   name
    // delete game
    Ok(())
}
