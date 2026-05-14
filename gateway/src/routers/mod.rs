

use axum::Router;
use axum::http::Method;
use axum::routing::{get, post, put, delete};
use tower_http::cors::{Any, CorsLayer};


use crate::handlers::test_handler;


pub fn app_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/", get(test_handler))
        .layer(cors)

}

