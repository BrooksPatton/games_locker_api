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
                    .col(ColumnDef::new(PlayerGames::PlayerId).integer().not_null())
                    .col(ColumnDef::new(PlayerGames::GameId).integer().not_null())
                    .col(
                        ColumnDef::new(PlayerGames::Finished)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(PlayerGames::Goal).text().not_null())
                    .col(
                        ColumnDef::new(PlayerGames::Ignoring)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PlayerGames::Playing)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(PlayerGames::Replay)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlayerGames::Table, PlayerGames::PlayerId)
                            .to(Players::Table, Players::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PlayerGames::Table, PlayerGames::GameId)
                            .to(Games::Table, Games::Id),
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

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PlayerGames {
    Table,
    Id,
    PlayerId,
    GameId,
    Finished,
    Goal,
    Ignoring,
    Playing,
    Replay,
}

#[derive(Iden)]
enum Games {
    Table,
    Id,
}

#[derive(Iden)]
enum Players {
    Table,
    Id,
}
