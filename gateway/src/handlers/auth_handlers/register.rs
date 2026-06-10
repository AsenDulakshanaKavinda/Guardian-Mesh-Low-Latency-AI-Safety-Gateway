use axum::{Extension, Json, http::StatusCode};

use uuid::Uuid;

use serde::Deserialize;
use validator::Validate;

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{
    entities,
    handlers::auth_handlers::AuthResponse,
    security::password_mngr::get_hashed_password,
    utils::{errors::AppError, jwt::create_jwt},
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterModel {
    #[validate(length(min = 3))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

// todo: Chnage the error type after dev
pub async fn register_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<RegisterModel>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let existing_user = entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|err| AppError::Internal(err.into()))?;

    if existing_user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    let hashed_password = get_hashed_password(&user_data.password)?;

    let user_id = Uuid::new_v4();

    let user = entities::user::ActiveModel {
        user_id: Set(user_id.into()),
        username: Set(user_data.username),
        email: Set(user_data.email),
        password_hash: Set(hashed_password),
        // created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    // todo: Chnage the error type after dev
    user.insert(&db)
        .await
        .map_err(|err| AppError::Internal(err.into()))?;

    // todo: Chnage the error type after dev
    let token = create_jwt(user_id.to_string()).map_err(|err| AppError::Internal(err.into()))?;

    Ok((StatusCode::CREATED, Json(AuthResponse { token })))
}
