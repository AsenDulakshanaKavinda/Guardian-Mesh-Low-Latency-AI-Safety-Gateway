use axum::{
    Router, 
    routing::post
};

use crate::handlers::prompt_hadlers::prompt_hadler::insert_prompt;

pub fn prompt_routes() -> Router {

    Router::new()
        .route("/prompt/create", post(insert_prompt))
}
