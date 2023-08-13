use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Games {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Tags {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum GameStates {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Players {
    Table,
    Id,
    Nickname,
    Auth0Id,
}

#[derive(Iden)]
pub enum PlayerGames {
    Table,
    Id,
    GameId,
    PlayerId,
    GameStateId,
    Goal,
    Completed,
}

#[derive(Iden)]
pub enum GameTags {
    Table,
    Id,
    TagId,
    GameId,
}
