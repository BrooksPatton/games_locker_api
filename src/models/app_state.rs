use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
