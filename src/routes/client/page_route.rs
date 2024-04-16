use std::sync::{Arc};
use axum::extract::{Path, State};
use axum::Json;
use mongodb::bson;
use axum::http::StatusCode;
use mongodb::bson::spec::BinarySubtype;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::OffsetDateTime;
use tracing::info;
use crate::AppState;
use crate::repository::page_repository::PageRepo;
use crate::structs::account::Account;
use crate::structs::metadata::{BinaryMetadata, Metadata};
use crate::structs::page::Page;
use crate::structs::user::User;

pub async fn create_page(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>){
   app_state.db.create_new_page(uuid.to_string()).await;
}

pub async fn edit_page(State(app_state): State<Arc<AppState>>, Path((id, title)): Path<(String, String)>) {
    app_state.db.edit_page(id, title.clone()).await;

    info!("Updated page title {:?}", title);
}

pub async fn load_all_page(State(app_state): State<Arc<AppState>>, path: Option<Path<String>>) -> String {
    let uuid = path.unwrap().0;

    let page = app_state.db.load_all_pages(uuid).await;

    json!(page).to_string()
}

pub async fn bread_crumb(State(app_state): State<Arc<AppState>>, Json(uuid): Json<String>) -> Json<Value> {

    Json(json!({"value": "value" }))
}

// pub async fn load_page(State(app_state): State<Arc<AppState>>, Json(uuid): Json<String>) -> Json<Value> {
//     let result = app_state.db.load_page(uuid).await;
//
//     match result {
//         None => {
//             Json(json!({"status": "failed" }))
//         }
//         Some(value) => {
//
//             Json(json!({"value": "asdffsda" }))
//         }
//     }
// }

//TODO make load a page


pub async fn delete_page(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>){

    // if let Some(owner_uuid) = page.owner.unwrap().uuid {
    //     map.remove(&owner_uuid); // Pass a reference to the key
    // }
}