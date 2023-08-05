use games_locker::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();

    match server.serve().await {
        Ok(_) => println!("Server exited"),
        Err(error) => eprintln!("Server exited with error: {error}"),
    }
}
