use crate::types::user::User;
use entity::users::ActiveModel;
use eyre::Result;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, TryIntoModel};

pub async fn insert_user(db: &DatabaseConnection, user: &User) -> Result<User> {
    Ok(ActiveModel {
        auth0_id: Set(user.get_auth0_id()?.to_owned()),
        email: Set(user.get_email()?.to_owned()),
        nickname: Set(user.get_nickname()?.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await?
    .try_into_model()?
    .into())
}
