use axum::{Extension, Json, http::StatusCode};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{entities, handlers::auth_handlers::AuthResponse, utils::{errors::AppError, jwt::create_jwt, password::hash_password}};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterModel {

    #[validate(length(min = 3))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}


pub async fn register_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<RegisterModel>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {

    let existing_user = entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if existing_user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    let hashed_password = hash_password(&user_data.password)?;

    let user_id = Uuid::new_v4();

    let user = entities::user::ActiveModel {
        user_id: Set(user_id.to_string()),
        username: Set(user_data.username),
        email: Set(user_data.email),
        password: Set(hashed_password),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user.insert(&db)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let token = create_jwt(user_id.to_string())
        .map_err(|_| AppError::InternalServerError)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse { token }),
    ))

}