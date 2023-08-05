use crate::models::player::Player;

use super::AppResponse;
use axum::http::StatusCode;

pub async fn create_player() -> AppResponse {
    AppResponse::new().status(StatusCode::CREATED)
}

pub async fn login() -> AppResponse {
    let player = Player::default();

    AppResponse::new().set_data(player)
}
