
mod server;
mod routers;
mod handlers;

#[tokio::main]
async fn main() {
    server::server().await;
}