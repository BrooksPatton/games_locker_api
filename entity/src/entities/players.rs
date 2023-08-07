//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "players")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub auth0_id: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::player_games::Entity")]
    PlayerGames,
    #[sea_orm(has_many = "super::games::Entity")]
    Games,
}

impl Related<super::player_games::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayerGames.def()
    }
}

impl Related<super::games::Entity> for Entity {
    fn to() -> RelationDef {
        super::player_games::Relation::Games.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::player_games::Relation::Players.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}