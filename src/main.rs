use games_locker::Server;

#[tokio::main]
async fn main() {
    let server = match Server::new().await {
        Ok(server) => server,
        Err(error) => {
            eprintln!("Error setting up server: {error}");
            panic!();
        }
    };

    match server.serve().await {
        Ok(_) => println!("Server exited"),
        Err(error) => eprintln!("Server exited with error: {error}"),
    }
}
