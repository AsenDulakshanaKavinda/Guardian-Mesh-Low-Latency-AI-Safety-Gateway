use axum::{Router, routing::post};

use crate::handlers::auth_handlers::{login::login_handler, register::register_handler};

pub fn authentication_route() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
