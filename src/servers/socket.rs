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

use tracing::{error, info};
use crate::AppState;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {

    socket.on("message",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                  socket.join("qwerqwer").expect("TODO: panic message");
              }
    );

    socket.on("chatting",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                  socket.to("qwerqwer").emit("broadcast", "test").expect("TODO: panic message");
              }
    );
}