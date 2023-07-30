use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{api, config::Config, db::user_queries, types::user::User};

pub async fn create_user(
    state: State<Config>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let user = api::auth0::create_auth0_user(user).await.map_err(|error| {
        tracing::error!("Error creating user: {error}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error creating an account",
        )
    })?;
    let user = user_queries::insert_user(&state.db, &user)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting user into database: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating an account",
            )
        })?
        .build_created_user()
        .map_err(|error| {
            tracing::error!("Error inserting User: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user")
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}
