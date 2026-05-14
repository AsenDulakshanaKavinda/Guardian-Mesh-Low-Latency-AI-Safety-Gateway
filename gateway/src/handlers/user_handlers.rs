use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{entities::{self, user}, models::user_models::CreateUserModel, utils::errors::AppError};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>
) -> Result<StatusCode, AppError>{
    // check if user already exist or not
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    match user {
        Some(_user) => {
            return Err(AppError::UserAlreadyExists)
        },
        None => {
            let new_user = entities::user::ActiveModel {
                username: Set(user_data.username.to_owned()),
                email: Set(user_data.email.to_owned()),
                password: Set(user_data.password.to_owned()),
                ..Default::default() 
            };

            new_user.insert(&db)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(StatusCode::CREATED)
        }
    }
}

pub async fn fetch_user(

) {
    todo!();
}

pub async fn update_user(

) {
    todo!();
}

pub async fn detele_user(

) {
    todo!();
}