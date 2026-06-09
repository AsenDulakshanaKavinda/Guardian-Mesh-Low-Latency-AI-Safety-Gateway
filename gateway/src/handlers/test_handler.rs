use axum::response::IntoResponse;

pub async fn test_handler() -> impl IntoResponse {
    "Hello, World!"
}
