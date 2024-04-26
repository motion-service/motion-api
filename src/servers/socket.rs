use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use axum::http::{Method, Uri};
use axum::Json;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::{Error, ObjectId};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use futures::TryStreamExt;
use serde_json::{json, Value};
use socketioxide::extract::State;

use tracing::{error, info};
use crate::AppState;
use crate::db::database::Database;
use crate::repository::block_repository::BlockRepository;
use crate::structs::metadata::Metadata;
use crate::structs::page::PageBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Ord, PartialOrd, Eq)]
pub struct TestBlock {
    #[serde(rename = "_id")]
    pub(crate) id: Option<ObjectId>,
    pub(crate) value: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomData {
    id: String,
    test_blocks: TestBlock,
}

pub async fn on_connect(socket: SocketRef) {
    socket.on("join", |socket: SocketRef, Data::<String>(page_id), State(app_state): State<Arc<AppState>>| async move {
        let id_result = ObjectId::from_str(&page_id);

        match id_result {
            Ok(id) => {
                let result = app_state.db.page_doc_collection.find_one(doc! {"_id": id}, None).await;

                if result.is_ok() {
                    let new_id = ObjectId::new();

                    socket.join(page_id.clone()).unwrap();
                    socket.join(new_id.to_owned().to_string()).unwrap();
                    socket.within(page_id).within(new_id.to_string()).emit("connected", new_id.to_string()).unwrap();
                }
            }
            Err(_) => {}
        }
    });

    socket.on("load_data", |socket: SocketRef, (room): (Data<String>), State(app_state): State<Arc<AppState>>| async move {
        // let mut cursor = app_state.db.test_block_collection.find({ doc! {"roomId":"65fbcfc01e2367498a28a057"} }, None).await.unwrap();
        let test = json!({"blocks": vec!["asfafsf","fasafsfas"]});
        let id = ObjectId::from_str("662b3f99b49d7b5ec6b3ffc5").unwrap();
        let result = app_state.db.test_block_doc_collection.find_one(doc! {"_id": id}, None).await.ok().unwrap().unwrap();

        let test = result.get_array("value").ok().unwrap();

        socket.within(room.0.to_owned()).emit("retrieve_blocks", json!({"test": test})).unwrap();
        info!("result : {:?}", test)
    });

    socket.on("send_data", |socket: SocketRef, (data): (Data<Value>), State(app_state): State<Arc<AppState>>| async move {
        info!("data : {:?}", data.0);
    });
}