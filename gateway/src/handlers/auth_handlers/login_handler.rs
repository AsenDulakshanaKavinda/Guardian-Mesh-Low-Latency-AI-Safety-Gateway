use axum::{
    Extension, 
    Json
};
use sea_orm::{
    ColumnTrait, 
    DatabaseConnection, 
    EntityTrait, 
    QueryFilter
};
use serde::{Deserialize};

use crate::{
    entities, 
    handlers::auth_handlers::AuthResponse, 
    utils::{
        errors::AppError, 
        jwt::create_jwt, 
        password::verify_password
    }
};


#[derive(Debug, Deserialize)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}


pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Json(login_data): Json<LoginModel>,
) -> Result<Json<AuthResponse>, AppError> {

    
    let user = entities::user::Entity::find()
    .filter(entities::user::Column::Email.eq(login_data.email))
    .one(&db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?
    .ok_or(AppError::InvalidCredentials)?;

    let valid = verify_password(
        &login_data.password,
        &user.password
    )?;

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(user.user_id.to_string())
        .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(AuthResponse { token }))
}