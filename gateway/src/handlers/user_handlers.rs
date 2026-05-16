
use axum::{Extension, Json, extract::Path, http::StatusCode};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use uuid::Uuid;
use validator::Validate;

use crate::{entities::{self, user}, models::{APIResponse, UserResponse, user_models::{CreateUserModel, FetchUserModel, UpdateUserModel}}, utils::{errors::AppError, password::hash_password}};


// create user
pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> Result<(StatusCode, Json<APIResponse<UserResponse>>), AppError> {

    let _ = user_data
        .validate()
        .map_err(|_| AppError::ValidationError);


    let existing_user = entities::user::Entity::find()
    .filter(
        entities::user::Column::Email.eq(user_data.email.clone())
    )
    .one(&db)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    if existing_user.is_some() {
        return Err(AppError::UserAlreadyExists);
    }

    let hashed_password = hash_password(&user_data.password)?;

    let user_id = Uuid::new_v4().to_string();

    let new_user = entities::user::ActiveModel {
        user_id: Set(user_id),
        username: Set(user_data.username.clone()),
        email: Set(user_data.email.clone()),
        password: Set(hashed_password),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let inserted_user = new_user
        .insert(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let response = UserResponse {
        user_id: inserted_user.user_id,
        username: inserted_user.username,
        email: inserted_user.email,
        created_at: inserted_user.created_at,
    };

    Ok((
        StatusCode::CREATED,
        Json(APIResponse {
            success: true,
            message: "User created successfully".to_string(),
            data: Some(response),
        }),
    ))

}

// fetch user
pub async fn fetch_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<FetchUserModel>

) -> Result<(StatusCode, Json<APIResponse<UserResponse>>), AppError> {
    let user = entities::user::Entity::find()
        .filter(
            entities::user::Column::Email.eq(user_data.email.clone())
        )
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::UserNotFound)?;

    let response = UserResponse {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    };

    Ok((
        StatusCode::CREATED,
        Json(APIResponse {
            success: true,
            message: "User fetch successfully".to_string(),
            data: Some(response),
        }),
    ))
}


pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>,
) -> Result<(StatusCode, Json<APIResponse<UserResponse>>), AppError> {

    // validate request

    let user = entities::user::Entity::find_by_id(uuid)
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::UserNotFound)?;

    let mut user_active: entities::user::ActiveModel = user.into();

    user_active.username = Set(user_data.username);

    let updated_user = user_active
        .update(&db)
        .await
        .map_err(|err| AppError::Internal(err.into()))?;

    let response = UserResponse {
        user_id: updated_user.user_id,
        username: updated_user.username,
        email: updated_user.email,
        created_at: updated_user.created_at,
    };

    Ok((
        StatusCode::CREATED,
        Json(APIResponse {
            success: true,
            message: "User updated successfully".to_string(),
            data: Some(response),
        }),
    ))
}


pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {

    let result = entities::user::Entity::delete_by_id(uuid)
        .exec(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    if result.rows_affected == 0 {
        return Err(AppError::UserNotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}


