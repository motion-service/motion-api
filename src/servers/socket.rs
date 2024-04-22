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

    socket.on("connection",
              |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
                  socket.join("test").unwrap();
                  socket.emit("Joined","test"). unwrap();

              }
    );

    socket.on("chatting",
              |socket: SocketRef, Data::<Value>(data)| {
                  socket.within("test").emit("broadcast", "test").unwrap();
              }
    );
}