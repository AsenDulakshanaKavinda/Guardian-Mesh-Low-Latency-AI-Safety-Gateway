use axum::{Router, routing::{delete, get, post, put}};

use crate::handlers::user_handlers::{create_user, fetch_user, update_user, delete_user};


pub fn user_routes() -> Router {

    Router::new()
        .route("/user/create", post(create_user))
        // .route("/user/fetch/{uuid}", put(fetch_user))
        .route("/user/update/{uuid}", put(update_user))
        .route("/user/delete/{uuid}", delete(delete_user))

}
