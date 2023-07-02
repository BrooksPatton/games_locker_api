use axum::http::StatusCode;

pub async fn create_user() -> StatusCode {
    StatusCode::CREATED
}
