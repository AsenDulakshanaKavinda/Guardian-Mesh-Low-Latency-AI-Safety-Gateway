use axum::routing::{Router, get};

use crate::handlers::test_handler::test_handler;
 

pub async fn test_route() -> Router {
    Router::new().route("/test", get(test_handler))
}

