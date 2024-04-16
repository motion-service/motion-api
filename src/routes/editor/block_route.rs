use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use axum_client_ip::InsecureClientIp;
use serde_json::{json, Value};
use tracing::info;
use crate::AppState;
use crate::repository::block_repository::BlockRepository;

use crate::structs::account::Account;
use crate::structs::block_data::BlockData;
use crate::structs::content::BlockType;
use crate::structs::default_block::DefaultBlock;
use crate::structs::styles::TextDecorationType;

pub async fn create_block(State(app_state): State<Arc<AppState>>, path: Option<Path<String>>) -> String {
    // let block = app_state.db.add_block(uuid.uuid.unwrap()).await;

    app_state.db.add_new_block(path.unwrap().0).await;

    // app_state.db.create_block("").await;
    // let test = app_state.db.add_new_block(uuid.uuid.unwrap()).await;
    json!({}).to_string()
}

pub async fn delete_block(State(app_state): State<Arc<AppState>>, path: Option<Path<String>>) -> impl IntoResponse {

    Json(json!({"":""}))
}

pub async fn edit_block(State(app_state): State<Arc<AppState>>, Json(block_data): Json<BlockData>){
    app_state.db.edit_block_styles(block_data).await;
}

pub async fn load_blocks(State(app_state): State<Arc<AppState>>, Path((uuid)): Path<(String)>) -> Json<Value>{
    let blocks = app_state.db.load_all_block(uuid).await;

    Json(json!(blocks))
}
