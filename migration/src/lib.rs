pub use sea_orm_migration::prelude::*;

mod m20230802_015101_create_player;
mod m20230803_015122_create_games;
mod m20230803_015448_create_optimal_session_length;
mod m20230804_012906_create_player_games_table;
mod m20230804_015457_create_tag_table;
mod m20230804_015913_create_game_tags_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230802_015101_create_player::Migration),
            Box::new(m20230803_015448_create_optimal_session_length::Migration),
            Box::new(m20230803_015122_create_games::Migration),
            Box::new(m20230804_012906_create_player_games_table::Migration),
            Box::new(m20230804_015457_create_tag_table::Migration),
            Box::new(m20230804_015913_create_game_tags_table::Migration),
        ]
    }
}
