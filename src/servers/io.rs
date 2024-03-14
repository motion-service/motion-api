use std::str::FromStr;
use std::time::Duration;
use axum::extract::{Request};
use axum::http::{Method, Uri};
use socketioxide::{
    extract::{Bin, Data, SocketRef},
};
    
use serde_json::Value;
use serde_json::Value::String;
use tracing::{error, info};

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

    socket.on("client-ready",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                  info!("Ready");
              },
    );

    socket.on("message",
        |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
            info!("Received event test: {:?} {:?}", data, bin);


            Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header("content-type", "application/json")
                .body(serde_json::to_vec(&data).unwrap())
                .unwrap();

            socket.timeout(Duration::from_secs(2)).broadcast().emit("message-back", data).ok();
        },
    );
}