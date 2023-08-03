pub use sea_orm_migration::prelude::*;

mod m20230802_015101_create_player;
mod m20230803_015122_create_games;
mod m20230803_015448_create_optimal_session_length;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230802_015101_create_player::Migration),
            Box::new(m20230803_015448_create_optimal_session_length::Migration),
            Box::new(m20230803_015122_create_games::Migration),
        ]
    }
}
