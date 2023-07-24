use games_locker::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    match games_locker::run(config).await {
        Ok(_) => println!("Server shut down successfully"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
