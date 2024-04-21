use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use axum::extract::{Request, State};
use axum::http::{Method, Uri};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};

use serde_json::Value;
use serde_json::Value::String;
use tracing::{error, info};
use crate::AppState;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    let ip_address = std::env::var("IP_ADDRESS").expect("IP_ADDRESS must be set.");

    let uri = {
        let mut uri = ip_address.clone();
        uri.push_str("/data");
        uri
    };

    let uri = match Uri::from_str(&uri) {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to parse URI: {:?}", e);
            return;
        }
    };

    socket.on("join",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {

                  info!("Ready ");
              },
    );

    socket.on("message",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                  let test = data.clone();

                  info!("Received event test: {:?} {:?}", data, bin);

                  Request::builder()
                      .method(Method::POST)
                      .uri(uri)
                      .header("content-type", "application/json")
                      .body(serde_json::to_vec(&data).unwrap())

              },
    );

}