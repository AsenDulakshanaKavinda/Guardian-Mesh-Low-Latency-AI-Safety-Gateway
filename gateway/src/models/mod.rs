use serde::{Deserialize, Serialize};


pub mod user_models;
pub mod auth_models;

#[derive(Debug, Serialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}


