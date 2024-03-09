// src/app.rs
use std::net::SocketAddr;
use crate::servers;

pub struct App {
    port: SocketAddr,
}

impl App {
    pub fn new(p: SocketAddr) -> Self {
        Self { port: p }
    }
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        servers::web::Server::run().await
    }
}