pub mod types;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{api, config::Config, db::user_queries};

use self::types::{CreateAuth0User, User};

pub async fn create_user(
    state: State<Config>,
    Json(create_auth0_user): Json<CreateAuth0User>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let user = api::auth0::create_auth0_user(create_auth0_user.into())
        .await
        .map_err(|error| {
            tracing::error!("Error creating user: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating an account",
            )
        })?;
    let user = user_queries::insert_user(&state.db, &user._id, &user.email, &user.email)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting user into database: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating an account",
            )
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}
