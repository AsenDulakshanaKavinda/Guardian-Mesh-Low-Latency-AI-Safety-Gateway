use serde::Serialize;

pub mod register_handler;
pub mod login_handler;
pub mod helper;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}