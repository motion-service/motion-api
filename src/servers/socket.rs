use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use axum::http::{Method, Uri};
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};

use serde_json::Value;
use socketioxide::extract::State;

use tracing::{error, info};
use crate::AppState;
use crate::db::database::Database;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct TestBlock {
    #[serde(rename = "_id")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) value: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomData {
    id: String,
    test_blocks: TestBlock
}

pub async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        "join",
        |socket: SocketRef, Data::<String>(room), State(app_state): State<Arc<AppState>>| async move {

        },
    );
}