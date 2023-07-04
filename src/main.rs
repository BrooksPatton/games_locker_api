use dotenvy_macro::dotenv;
use eyre::Result;
use games_locker::config::Config;
use games_locker::run;
use migration::MigratorTrait;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT");
    let db_uri = dotenv!("DB_URI");
    let db = Database::connect(db_uri)
        .await
        .expect("Error connecting to the database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Error running migration");
    let config = match Config::new(port, db) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error creating config: {:?}", error);
            return;
        }
    };

    match run(&config).await {
        Ok(_) => println!("server closed"),
        Err(error) => eprintln!("Server closed due to error: {:?}", error.to_string()),
    };
}
