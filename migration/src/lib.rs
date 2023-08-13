pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230813_190143_create_tags_table;
mod m20230813_190550_create_game_states_table;
mod m20230813_190758_create_players_table;
mod m20230813_191055_create_player_games_table;
mod m20230813_193055_create_game_tags_table;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230813_190143_create_tags_table::Migration),
            Box::new(m20230813_190550_create_game_states_table::Migration),
            Box::new(m20230813_190758_create_players_table::Migration),
            Box::new(m20230813_191055_create_player_games_table::Migration),
            Box::new(m20230813_193055_create_game_tags_table::Migration),
        ]
    }
}
