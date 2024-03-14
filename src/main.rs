#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing_subscriber::FmtSubscriber;
use crate::app::App;
use crate::db::db::DB;
use crate::structs::page::Page;
use crate::structs::user::User;

mod servers;
mod structs;
mod routes;
mod db;
mod error;
mod app;

//TODO create list variable on AppState with contains Binary struct
pub struct AppState {
    db: DB,
    page: Arc<Mutex<HashMap<String, Vec<Page>>>>,
    users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

    dotenv::dotenv().ok();

    //TODO create routes for SocketIO that path name called "socket"
    //TODO received data from the client via `on` method, than try to fetch data to the "socket" route
    //TODO "socket" route should be check if that page url doc is existed on the mongodb, than if it's existed than saved the data

    let app = App::new(SocketAddr::from(([0, 0, 0, 0], 8090)));
    app.run().await.expect("TODO: panic message");
}
