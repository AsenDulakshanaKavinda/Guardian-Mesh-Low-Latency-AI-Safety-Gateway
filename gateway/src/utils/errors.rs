use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;



#[derive(Debug)]
enum AppError {
    InvalidToken,
    MissingCredentials,
    UserNotFound,
    Conflict(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token".to_string()),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials".to_string()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Self::Internal(err) => {
                println!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}



