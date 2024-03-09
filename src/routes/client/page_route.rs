use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum_client_ip::InsecureClientIp;
use std::borrow::Borrow;
use mongodb::bson;
use mongodb::bson::{doc, Bson, Binary, DateTime as BsonDateTime, datetime, Document};
use std::convert::TryInto;
use std::time::SystemTime;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::spec::BinarySubtype;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::OffsetDateTime;
use tracing::info;
use uuid::{Error, Uuid};
use crate::AppState;
use crate::structs::account::Account;
use crate::structs::metadata::Metadata;
use crate::structs::page::Page;
use crate::structs::user::User;

pub async fn create_page(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(uuid): Json<String>) -> Json<Value> {
    let uuid_string = uuid.clone();

    let parse_result = Uuid::parse_str(&uuid_string.clone());

    match parse_result {
        Ok(uuid) => {
            let userResult = app_state.db.user_collection.find_one(doc! {"uuid": uuid_string}, None).await.expect("");
            match userResult {
                None => {
                    Json(json!({"status": "failed" }))
                }
                Some(user) => {
                    let user_doc = bson::to_document(&user).expect("Failed to serialize user");

                    let uuid = Uuid::new_v4();
                    let metadata = bson::to_document(&Metadata::new(uuid)).unwrap();

                    let now = OffsetDateTime::now_local().unwrap().to_string();
                    let uuid = user.uuid.unwrap();

                    app_state.db.page_doc_collection.insert_one(doc! {
                        "owner": uuid,
                        "metadata": metadata.clone(),
                        "isShared": false,
                        "created": now
                    }, None).await.expect("test");

                    Json(json!({"value": metadata }))
                }
            }
        }
        Err(_) => {
            Json(json!({"status": "parse_failed" }))
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataData{
    pub(crate) url: Option<Uuid>,
    pub(crate) title: Option<String>,
}

pub async fn edit_page(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(metadata): Json<MetadataData>) -> Json<Value> {
    let url = metadata.url.unwrap().to_string();

    let filter = doc! {"metadata.url": url.clone()};
    let doc_result = app_state.db.page_doc_collection.find_one(filter.clone(), None).await.expect("TODO: panic message");

    match doc_result {
        None => {
            Json(json!({"status": "not_found_doc" }))
        }
        Some(doc) => {

            let update_doc = doc! {
                "$set": {
                    "metadata.title": metadata.title.unwrap()
                }
            };
            app_state.db.page_doc_collection.update_one(filter, update_doc, None).await.expect("TODO: panic message");
            Json(json!({"status": "test" }))
        }
    }
}

pub async fn load_all_page(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(uuid): Json<String>) -> Json<Value> {
    let value = app_state.db.load_all_pages(uuid).await;
    info!("{:?}", value);
    Json(json!({"value": value }))
}

pub async fn bread_crumb(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(uuid): Json<String>) -> Json<Value> {
    let value = app_state.db.load_all_pages(uuid).await;
    info!("{:?}", value);
    Json(json!({"value": value }))
}

pub async fn load_page(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(uuid): Json<String>) -> Json<Value> {
    let result = app_state.db.load_page(uuid).await;

    match result {
        None => {
            Json(json!({"status": "failed" }))
        }
        Some(value) => {

            Json(json!({"value": json!(value) }))
        }
    }
}

//TODO make load a page


pub async fn delete_page(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(page): Json<Page>){
    let map = app_state.page.clone();
    let mut map = map.write().expect("Failed to obtain a write lock on the map");

    if let Some(owner_uuid) = page.owner.unwrap().uuid {
        map.remove(&owner_uuid); // Pass a reference to the key
    }
}