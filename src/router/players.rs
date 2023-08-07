use crate::{
    api::{self, auth0::signup},
    models::{
        app_state::AppState,
        player::{Auth0PlayerLogin, Auth0SignupPlayer, Player, PlayerError, UpsertPlayer},
    },
};

use super::AppResponse;
use axum::{extract::State, http::StatusCode, Json};

pub async fn create_player(Json(player): Json<Player>) -> Result<StatusCode, (StatusCode, String)> {
    let player = Auth0SignupPlayer::try_from(player).map_err(|error| {
        tracing::error!("{}", &error);
        (StatusCode::BAD_REQUEST, format!("{error}"))
    })?;

    signup(&player).await.map_err(|error| {
        tracing::error!("{}", &error);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
    })?;
    Ok(StatusCode::CREATED)
}

pub async fn login(
    app_state: State<AppState>,
    Json(player): Json<Player>,
) -> Result<Json<Player>, (StatusCode, String)> {
    let player = match Auth0PlayerLogin::try_from(player) {
        Ok(player) => player,
        Err(error) => {
            tracing::error!("Error creating Auth0PlayerLogin: {error}");
            return Err((StatusCode::BAD_REQUEST, format!("{error}")));
        }
    };
    let player = api::auth0::login(&player)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")))?;
    let player = UpsertPlayer::try_from(player).map_err(|error| {
        tracing::error!("Error converting into upsert player: {error}");
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
    })?;

    Ok(Json(player))
}
