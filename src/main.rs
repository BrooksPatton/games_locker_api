use dotenvy_macro::dotenv;
use games_locker::config::Config;
use games_locker::run;

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
    match run(&config).await {
        Ok(_) => println!("server closed"),
        Err(error) => eprintln!("Server closed due to error: {:?}", error.to_string()),
    }
}
