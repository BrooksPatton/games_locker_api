use crate::{api, AppState};
use axum::http::StatusCode;
use axum::response::Result;
use axum::{extract::State, Json};
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};

pub async fn create_user(
    State(state): State<AppState>,
    Json(new_user): Json<NewUserJson>,
) -> Result<StatusCode> {
    let auth0_user =
        api::users::create_auth0_user(&new_user.email, &new_user.nickname, &new_user.password)
            .await
            .map_err(|error| error.to_string())?;

    let db_user = entity::users::ActiveModel {
        auth0_id: Set(auth0_user.id),
        nickname: Set(auth0_user.nickname),
        email: Set(auth0_user.email),
        ..Default::default()
    };
    db_user
        .save(&state.db)
        .await
        .map_err(|error| error.to_string())?;
    Ok(StatusCode::CREATED)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserJson {
    pub email: String,
    pub password: String,
    pub nickname: String,
}
