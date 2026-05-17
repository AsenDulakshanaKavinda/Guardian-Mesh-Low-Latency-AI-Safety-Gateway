use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    // Authentication and Authorization Errors
    InvalidToken,
    MissingCredentials,
    UserNotFound,
    UserAlreadyExists,
    HashingError,
    Unauthorized,
    ValidationError,
    InvalidCredentials,

    // User-related Errors

    // General Errors

    // Conflict Errors
    Conflict(String),

    // Internal Errors
    InternalServerError,
    Internal(anyhow::Error),

    // grpc clinet errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            ),
            Self::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            Self::UserAlreadyExists => (StatusCode::NOT_FOUND, "User already exist".to_string()),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Self::HashingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while hashing the password".to_string(),
            ),
            Self::ValidationError => (StatusCode::CONFLICT, "Validation failed".to_string()),
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, msg),
            Self::Internal(err) => {
                println!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".into(),
                )
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type Result<T> = core::result::Result<T, AppError>;
