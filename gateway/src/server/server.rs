use std::net::SocketAddr;



use crate::routers::{main_route, test_route::test_route};




pub async fn setup_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let _main_route = main_route().await;


    println!(">>> Listening on - {addr} \n");

    axum::serve(listener, _main_route).await.unwrap();
}
