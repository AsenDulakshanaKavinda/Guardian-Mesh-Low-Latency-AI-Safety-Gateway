

use axum::{Extension, Router};
use axum::http::Method;
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};

mod auth_route;
mod user_route;
//vuse crate::routes::auth_route::auth_routes;
use crate::routes::user_route::user_routes;
use crate::utils;

pub async fn main_route() -> Router {
    let conn_str = (*utils::constants::DATABASE_URL).clone();

    let db: DatabaseConnection = Database::connect(conn_str)
        .await
        .expect("Error while connecting to database.");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let main_route = Router::new()
        // .merge(auth_routes())
        .merge(user_routes())
        .layer(Extension(db))
        .layer(cors);

    main_route
}




