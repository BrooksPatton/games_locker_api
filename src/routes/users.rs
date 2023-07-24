pub mod types;

use axum::{extract::State, http::StatusCode, Json};

use crate::config::Config;

use self::types::CreateAuth0User;

pub async fn create_user(
    state: State<Config>,
    Json(new_user): Json<CreateAuth0User>,
) -> StatusCode {
    let url = format!("https://{}/dbconnections/signup", state.auth0_domain);
    StatusCode::CREATED
}
