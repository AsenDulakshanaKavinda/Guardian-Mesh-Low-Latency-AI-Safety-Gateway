
mod server;
mod routers;
mod handlers;
mod utils;



#[tokio::main]
async fn main() {

    server::server().await;
}