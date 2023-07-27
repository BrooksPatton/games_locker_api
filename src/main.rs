use games_locker::config::Config;

#[tokio::main]
async fn main() {
    let config = match Config::new().await {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error creating config: {error}");
            return;
        }
    };
    match games_locker::run(config).await {
        Ok(_) => println!("Server shut down successfully"),
        Err(error) => eprintln!("Server crashed with error: {error}"),
    }
}
