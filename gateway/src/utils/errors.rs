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
    InvalidCredentials,
    Unauthorized,
    Forbidden, // Added: Authenticated, but lacks permissions

    // User-related Errors
    UserNotFound,
    UserAlreadyExists,

    // General Client Errors
    BadRequest(String), // Generic catch-all for bad client input
    ValidationError(String), // Added payload to describe what failed validation
    NotFound(String),   // Generic 404 for other resources

    // Conflict Errors
    Conflict(String),

    // Internal & Database Errors
    HashingError,
    DatabaseError(String), // Specifically for tracking DB issues internally
    InternalServerError,
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // --- Authentication and Authorization ---
            Self::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            ),
            Self::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".to_string())
            }
            Self::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED, 
                "Authentication required".to_string()
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                "You do not have permission to access this resource".to_string(),
            ),

            // --- User-related Errors ---
            Self::UserNotFound => (
                StatusCode::NOT_FOUND, 
                "User not found".to_string()
            ),
            Self::UserAlreadyExists => (
                StatusCode::CONFLICT, // Corrected from NOT_FOUND to CONFLICT
                "User already exists".to_string(),
            ),

            // --- General Client Errors ---
            Self::BadRequest(msg) => (
                StatusCode::BAD_REQUEST, 
                msg
            ),
            Self::ValidationError(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY, // Corrected from CONFLICT to 422
                format!("Validation failed: {}", msg),
            ),
            Self::NotFound(msg) => (
                StatusCode::NOT_FOUND, 
                msg
            ),

            // --- Conflict Errors ---
            Self::Conflict(msg) => (
                StatusCode::CONFLICT, 
                msg
            ),

            // --- Internal Server & Database Errors ---
            Self::HashingError => {
                // Log the real issue on the server side
                tracing::error!("Password hashing failed"); 
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Self::DatabaseError(err) => {
                // Log the database error internally, don't leak it to the client
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            Self::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type Result<T> = core::result::Result<T, AppError>;