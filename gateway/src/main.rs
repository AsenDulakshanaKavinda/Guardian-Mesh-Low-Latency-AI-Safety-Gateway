use crate::server::server::setup_server;

mod utils;
mod server;
mod routers;
mod handlers;
mod security;
mod schemas;
mod entities;


#[tokio::main]
async fn main() {
    setup_server().await;
}
