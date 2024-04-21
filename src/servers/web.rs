use std::net::SocketAddr;
use std::sync::{Arc};

// src/server.rs
use axum::{routing::get, ServiceExt};
use axum::routing::{delete, post};
use axum_client_ip::SecureClientIpSource;
use serde_json::Value;
use socketioxide::extract::SocketRef;
use socketioxide::SocketIo;

use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use crate::AppState;
use crate::db::database::Database;
use crate::routes::account::auth_route::{get_account, login_account, register_account};
use crate::routes::client::editor_route::get_shared;
use crate::routes::client::page_route::{bread_crumb, create_page, delete_page, edit_page, load_all_page};
use crate::routes::editor::block_route::{create_block, delete_block, edit_block, load_blocks};
use crate::routes::editor::style_route::{edit_style, load_style};
use crate::routes::editor::user_route::{get_guests, get_members};
use crate::servers::socket::on_connect;

pub struct Server {}

impl Server {
    //TODO check out https://github.com/keyosk/alien-metrics/blob/d8882dafac2cc2993582211960c673c1efa62882/src/main.rs#L92
    pub async fn run(){
        let db = Database::init().await;
        let (layer, io) = SocketIo::builder().build_layer();

        io.ns("/", on_connect);


        let app_state = Arc::new(AppState {
            db: db.clone(),
        });

        let app = axum::Router::new()
            .route("/", get(|| async { "Hello, World!" }))

            .route("/auth/register", post(register_account))
            .route("/auth/login", post(login_account))
            .route("/auth/account/:uuid", get(get_account))

            .route("/client/page/load/all/:uuid", post(load_all_page))
            // .route("/client/page/load/bread_crumb", post(bread_crumb))
            // .route("/client/page/load/single/:uuid", post(load_page))
            .route("/client/page/create/:uuid", post(create_page))
            .route("/client/page/edit/:id/:title/", post(edit_page))
            .route("/client/page/delete/:uuid", delete(delete_page))

            .route("/editor/block/create/:uuid", post(create_block))
            .route("/editor/block/delete/:uuid", delete(delete_block))
            .route("/editor/block/load/:uuid", post(load_blocks))
            .route("/editor/block/style/edit/:page_id/:index/:start/:end", post(edit_style))
            .route("/editor/block/style/load/:page_id/:index/:start/:end", post(load_style))

            .route("/editor/users/members", post(get_members))
            .route("/editor/users/guests", post(get_guests))

            .route("/editor/option/page/font", post(edit_page))
            .route("/editor/option/page/is_shared", post(get_shared))

            .with_state(app_state)
            .layer(layer)
            .layer(CookieManagerLayer::new())
            .layer(CorsLayer::permissive())
            .layer(ServiceBuilder::new()
                .layer(CorsLayer::permissive())
            .layer(SecureClientIpSource::ConnectInfo.into_extension()));

        //TODO https://medium.com/intelliconnect-engineering/securing-apis-with-apikey-39d0e22f62dd
        info!("Starting server");

        //TODO Checkout https://doc.rust-lang.org/rust-by-example/macros/designators.html

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    }
}