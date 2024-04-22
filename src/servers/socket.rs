use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use axum::extract::{Request, State};
use axum::http::{Method, Uri};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};

use serde_json::Value;

use tracing::{error, info};
use crate::AppState;
use crate::db::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBlock {
    value: String
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomData {
    id: String,
    test_blocks: Vec<TestBlock>
}

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {

    socket.on("connection",
              |socket: SocketRef, Data::<RoomData>(room), Bin(bin)| async move {
                  socket.join(room.clone().id).unwrap();

                  let database = Database::init().await;

                  info!("connected to {:?}",  room.id);
              }
    );

    socket.on("chatting",
              |socket: SocketRef, Data::<String>(id)| {
                  socket.within(id).emit("broadcast", "test").unwrap();
              }
    );
}