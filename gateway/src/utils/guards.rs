use axum::{
    body::Body, extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::prelude::Uuid;
use crate::{
    entities,
    utils::{self, errors::AppError, jwt::Claims},
};


#[derive(Clone)]
pub struct CurrentUser(pub entities::user::Model);

pub async fn guard(mut req: Request<Body>, next: Next) -> Result<Response, AppError> {
    let secret = (*utils::constants::JWT_SECRET).clone();

    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| AppError::Unauthorized)?;

    let db = req
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(AppError::InternalServerError)?
        .clone();


    let user = entities::user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|e| AppError::Internal(e.into()))?
        .ok_or(AppError::Unauthorized)?;

    req.extensions_mut().insert(CurrentUser(user));

    Ok(next.run(req).await)
}