use axum::{http::StatusCode, response::IntoResponse};


pub async fn test_handler() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Home").into_response()
}

