use serde::Serialize;

pub mod register_handler;
pub mod login_handler;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}