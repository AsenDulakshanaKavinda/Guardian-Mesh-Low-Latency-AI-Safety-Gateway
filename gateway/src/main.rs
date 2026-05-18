
mod entities;
mod handlers;
mod models;
mod routes;
mod server;
mod utils;
mod services;

mod generated;

#[tokio::main]
async fn main() {
    server::server().await;

}
