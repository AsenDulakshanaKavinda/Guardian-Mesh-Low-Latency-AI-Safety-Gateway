use axum::{Router, routing::{post}};

use crate::handlers::auth_handlers::login_handler::login_user;
use crate::handlers::auth_handlers::register_handler::register_user;


pub fn auth_routes() -> Router {

    Router::new()
        .route("/user/login", post(login_user))
        .route("/user/register", post(register_user))
}
