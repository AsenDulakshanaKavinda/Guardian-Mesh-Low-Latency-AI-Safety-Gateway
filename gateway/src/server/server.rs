use std::net::SocketAddr;



use crate::routers::{test_route::test_route};

use axum::{Json, Router, routing::get};
// - frontend test -
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn get_hello() -> Json<Message> {
    Json(Message { text: "Hello from the backend".to_string() })
}

pub async fn setup_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let _test_route = test_route().await;
    let _frontend = Router::new().route("/api/hello", get(get_hello));


    println!(">>> Listening on - {addr} \n");

    axum::serve(listener, _frontend).await.unwrap();
}
