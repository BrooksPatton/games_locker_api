mod players;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use players::{create_player, login};
use serde::{Deserialize, Serialize};

pub fn create_router() -> Router {
    Router::new()
        .route("/players", post(create_player))
        .route("/players/login", post(login))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

pub struct AppResponse {
    status: StatusCode,
    data: Option<Box<dyn erased_serde::Serialize>>,
}

impl AppResponse {
    pub fn new() -> Self {
        let status = StatusCode::OK;
        let data = None;

        Self { status, data }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn set_data(mut self, data: impl Serialize + 'static) -> Self {
        self.data = Some(Box::new(data));
        self
    }
}

impl Default for AppResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        if let Some(data) = self.data {
            (self.status, Json(data)).into_response()
        } else {
            self.status.into_response()
        }
    }
}
