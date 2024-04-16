#![forbid(unsafe_code)]
#![allow(unused_imports)]

use std::net::SocketAddr;
use tracing_subscriber::FmtSubscriber;
use crate::db::database::Database;

mod servers;
mod structs;
mod routes;
mod db;
mod error;
mod repository;
mod test;

pub struct AppState {
    db: Database
}

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

    dotenv::dotenv().ok();

    servers::web::Server::run().await;
}
