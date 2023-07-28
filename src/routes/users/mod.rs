pub mod types;

use axum::{extract::State, http::StatusCode};

use crate::config::Config;

pub async fn create_user(state: State<Config>) -> StatusCode {
    let url = format!("https://{}/dbconnections/signup", state.auth0_domain);
    StatusCode::CREATED
}
