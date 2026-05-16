
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserModel {

    #[validate(length(min = 3, max = 30))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

/* 
#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
}
*/

#[derive(Serialize, Deserialize)]
pub struct FetchUserModel {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub username: String,
}