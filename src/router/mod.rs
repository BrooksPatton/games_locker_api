mod players;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use players::{create_player, login};
use serde::{Deserialize, Serialize};

use crate::models::app_state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/players", post(create_player))
        .route("/players/login", post(login))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state)
}

pub struct AppResponse {
    status: StatusCode,
    data: Option<Box<dyn erased_serde::Serialize>>,
    error: Option<String>,
}

impl AppResponse {
    pub fn new() -> Self {
        let status = StatusCode::OK;
        let data = None;
        let error = None;

        Self {
            status,
            data,
            error,
        }
    }

    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    pub fn set_data(&mut self, data: impl Serialize + 'static) {
        self.data = Some(Box::new(data));
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }
}

impl Default for AppResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        if let Some(error) = self.error {
            let status = if self.status.is_success() {
                StatusCode::INTERNAL_SERVER_ERROR
            } else {
                self.status
            };

            return (status, error).into_response();
        };

        if let Some(data) = self.data {
            (self.status, Json(data)).into_response()
        } else {
            self.status.into_response()
        }
    }
}
