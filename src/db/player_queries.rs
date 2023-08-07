use crate::models::player::{Player, UpsertPlayer};
use entity::entities::players::Entity as PlayerEntity;
use entity::entities::players::Model as PlayerModel;
use eyre::Result;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

pub async fn upsert_player(db: &DatabaseConnection, player: UpsertPlayer) -> Result<PlayerModel> {
    todo!()
}

async fn get_player_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<PlayerModel>> {
    let player = PlayerEntity::find_by_id(id).one(db).await?;
    Ok(player)
}
