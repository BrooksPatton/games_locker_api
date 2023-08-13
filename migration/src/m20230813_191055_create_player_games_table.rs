use super::tables::{GameStates, Games, PlayerGames, Players};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayerGames::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayerGames::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlayerGames::GameId).integer().not_null())
                    .col(ColumnDef::new(PlayerGames::PlayerId).integer().not_null())
                    .col(ColumnDef::new(PlayerGames::GameStateId).integer())
                    .col(ColumnDef::new(PlayerGames::Goal).string())
                    .col(
                        ColumnDef::new(PlayerGames::Completed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlayerGames::Table, PlayerGames::GameId)
                            .to(Games::Table, Games::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlayerGames::Table, PlayerGames::PlayerId)
                            .to(Players::Table, Players::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlayerGames::Table, PlayerGames::GameStateId)
                            .to(GameStates::Table, GameStates::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayerGames::Table).to_owned())
            .await
    }
}
