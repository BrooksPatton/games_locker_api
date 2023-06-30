use dotenvy_macro::dotenv;
use games_locker::config::Config;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT");
    let config = match Config::new(port) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error creating config: {:?}", error);
            return;
        }
    };
}
