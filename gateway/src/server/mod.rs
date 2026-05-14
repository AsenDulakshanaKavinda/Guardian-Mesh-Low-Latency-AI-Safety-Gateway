
use std::net::SocketAddr;
use crate::routers::app_routes;

pub async fn server() {

    // address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // main routes
    let main_routes = app_routes();

    println!(">>> Listening on - {addr} \n");

    axum::serve(listener, main_routes)
        .await
        .unwrap();

}
