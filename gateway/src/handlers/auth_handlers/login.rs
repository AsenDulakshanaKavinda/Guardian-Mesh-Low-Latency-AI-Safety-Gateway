use axum::{Extension, Json, http::StatusCode};
use serde::Deserialize;

use crate::{
    entities,
    handlers::auth_handlers::AuthResponse,
    security::password_mngr::verify_password,
    utils::{errors::AppError, jwt::create_jwt},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(login_data): Json<LoginModel>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(login_data.email))
        .one(&db)
        .await
        .map_err(|err| AppError::Internal(err.into()))?
        .ok_or(AppError::InvalidCredentials)?;

    let valid = verify_password(&login_data.password, &user.password_hash)?;

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(user.user_id.to_string())
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::ACCEPTED, Json(AuthResponse { token })))
}
