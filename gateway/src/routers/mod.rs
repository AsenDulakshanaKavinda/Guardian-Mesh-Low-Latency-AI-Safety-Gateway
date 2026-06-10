use crate::routers::auth_route::authentication_route;
use crate::utils::guards::guard;
use axum::http::Method;
use axum::{Extension, Router, middleware};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};

use crate::utils;

pub mod auth_route;

pub async fn main_route() -> Router {
    let conn_str = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(conn_str)
        .await
        .expect("Error while connecting to database.");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let main_route = Router::new()
        // other protected route
        .layer(middleware::from_fn(guard))
        // auth route
        .merge(authentication_route())
        .layer(Extension(db))
        .layer(cors);

    main_route
}
