use serde::Serialize;

pub mod register;
pub mod login;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}