use crate::routes::users::types::User;
use entity::users::ActiveModel;
use eyre::Result;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, TryIntoModel};

pub async fn insert_user(
    db: &DatabaseConnection,
    auth0_id: &str,
    email: &str,
    nickname: &str,
) -> Result<User> {
    Ok(ActiveModel {
        auth0_id: Set(auth0_id.to_owned()),
        email: Set(email.to_owned()),
        nickname: Set(nickname.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await?
    .try_into_model()?
    .into())
}
