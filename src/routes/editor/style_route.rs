use std::str::FromStr;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson::oid;
use serde_json::json;
use crate::AppState;
use crate::repository::style_repository::RepoStyle;
use crate::structs::block_data::BlockData;
use crate::structs::content::Content;

pub async fn edit_style(State(app_state): State<Arc<AppState>>, Path((page_id, index, start, end)): Path<(String, u32, usize, usize)>, Json(content): Json<Content>){
    let page_uuid = oid::ObjectId::from_str(page_id.as_str()).unwrap();
    // app_state.db.update_style(page_uuid, index, start, end, content..unwrap(), content.text.unwrap()).await;
}

pub async fn load_style(State(app_state): State<Arc<AppState>>, Path((page_id, index, start, end)): Path<(String, u32, usize, usize)>, Json(content): Json<Content>){
    let page_uuid = oid::ObjectId::from_str(page_id.as_str()).unwrap();
    // app_state.db.load_style(page_uuid, index, start, end, content.styles.unwrap(), content.text.unwrap()).await;

}
