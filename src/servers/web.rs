use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use std::time::Duration;
// src/server.rs
use axum::{extract::Path, response::{IntoResponse, Response}, routing::get, ServiceExt};
use axum::routing::{delete, post};
use axum_client_ip::SecureClientIpSource;
use futures_util::select;
use serde_json::json;
use socketioxide::SocketIo;
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing::instrument::WithSubscriber;
use crate::AppState;
use crate::db::db::DB;
use crate::routes::account::auth_route::{get_account, login_account, register_account};
use crate::routes::client::editor_route::get_shared;
use crate::routes::client::page_route::{bread_crumb, create_page, delete_page, edit_page, load_all_page, load_page};
use crate::routes::editor::block_route::{create_block, delete_block, edit_block, load_block};
use crate::routes::editor::user_route::{get_guests, get_members};

use crate::servers::io::on_connect;
use crate::structs::user::User;

pub struct Server {}

impl Server {
    //TODO check out https://github.com/keyosk/alien-metrics/blob/d8882dafac2cc2993582211960c673c1efa62882/src/main.rs#L92

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let (layer, io) = SocketIo::builder().build_layer();

        let db = DB::init().await.unwrap();

        let users = db.load_all_user().await;
        let pages = db.load_all_pages().await;

        for (key, page) in pages.clone().iter() {
            info!("key : {:?} page: {:?} " , key, page)
        }

        io.ns("/", on_connect);

        let app_state = Arc::new(AppState {
            db: db.clone(),
            page: Arc::new(Mutex::new(pages.to_owned())),
            users: Arc::new(Mutex::new(users.to_owned()))
        });

        let app_state_clone = app_state.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                // let cloned_users = {
                //     // Create a local copy of users outside the async block
                //     let original_users = app_state_clone.users.lock().unwrap();
                //     original_users.clone()
                // };
                //
                // info!("cloned users: {:?}", cloned_users);

                // Your logic here using cloned_users...

                interval.tick().await;
            }
        });

        let app = axum::Router::new()
            .route("/", get(|| async { "Hello, World!" }))

            .route("/auth/register", post(register_account))
            .route("/auth/login", post(login_account))
            .route("/auth/account/:uuid", get(get_account))

            .route("/client/page/load/all", post(load_all_page))
            .route("/client/page/load/bread_crumb", post(bread_crumb))
            .route("/client/page/load/single", post(load_page))
            .route("/client/page/create", post(create_page))
            .route("/client/page/delete", delete(delete_page))

            .route("/editor/block/create", post(create_block))
            .route("/editor/block/delete", delete(delete_block))
            .route("/editor/block/load", post(load_block))
            .route("/editor/block/edit", post(edit_block))

            .route("/editor/users/members", post(get_members))
            .route("/editor/users/guests", post(get_guests))

            .route("/editor/option/page/font", post(edit_page))
            .route("/editor/option/page/is_shared", post(get_shared))

            .with_state(app_state)
            .layer(CookieManagerLayer::new())
            .layer(CorsLayer::permissive())
            .layer(ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer))
            .layer(SecureClientIpSource::ConnectInfo.into_extension());

        //TODO https://medium.com/intelliconnect-engineering/securing-apis-with-apikey-39d0e22f62dd
        info!("Starting server");

        //TODO Checkout https://doc.rust-lang.org/rust-by-example/macros/designators.html

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

        Ok(())
    }
}