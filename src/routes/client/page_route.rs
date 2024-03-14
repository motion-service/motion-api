use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum_client_ip::InsecureClientIp;
use std::borrow::Borrow;
use mongodb::bson;
use mongodb::bson::{doc, Bson, Binary, DateTime as BsonDateTime, datetime, Document};
use std::convert::TryInto;
use std::time::SystemTime;
use axum::http::StatusCode;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::spec::BinarySubtype;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::OffsetDateTime;
use tracing::info;
use uuid::{Error, Uuid};
use crate::AppState;
use crate::db::binary::BinaryUuid;
use crate::error::db_error::DbError;
use crate::structs::account::Account;
use crate::structs::metadata::Metadata;
use crate::structs::page::Page;
use crate::structs::user::User;

pub async fn create_page(State(app_state): State<Arc<AppState>>, Json(uuid): Json<BinaryUuid>) -> (StatusCode, String) {
    // let test = app_state.page.lock().unwrap();
   app_state.db.create_new_page(uuid.to_string()).await

}

pub async fn edit_page(State(app_state): State<Arc<AppState>>, Json(url): Json<Uuid>) -> (StatusCode, String) {
    // app_state.db.edit_page(Uuid::new_v4());


    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Cannot found page".to_string()
    )

    // app_state.db.edit_page()
    // let url = metadata.url.unwrap().to_string();
    //
    // let filter = doc! {"metadata.url": url.clone()};
    //
    //
    // match doc_result {
    //     None => {
    //         Json(json!({"status": "not_found_doc" }))
    //     }
    //     Some(doc) => {
    //
    //         let update_doc = doc! {
    //             "$set": {
    //                 "metadata.title": metadata.title.unwrap()
    //             }
    //         };
    //         app_state.db.page_doc_collection.update_one(filter, update_doc, None).await.expect("TODO: panic message");
    //         Json(json!({"status": "test" }))
    //     }
    // }
}

pub async fn load_all_page(State(app_state): State<Arc<AppState>>, Json(uuid): Json<BinaryUuid>) -> (StatusCode, String) {
    let page_lock = app_state.page.lock().unwrap();
    let pages = page_lock.get(&uuid.uuid.unwrap());

    match pages {
        None => {
            (
                StatusCode::BAD_REQUEST,
                "Cannot match".to_string()
            )
        }
        Some(new_pages) => {
            if new_pages.is_empty(){
                return (
                    StatusCode::OK,
                    "Empty".to_string()
                )
            }

            (
                StatusCode::OK,
                json!(new_pages).to_string()
            )
        }
    }
}

pub async fn bread_crumb(State(app_state): State<Arc<AppState>>, Json(uuid): Json<String>) -> Json<Value> {

    Json(json!({"value": "value" }))
}

pub async fn load_page(State(app_state): State<Arc<AppState>>, Json(uuid): Json<String>) -> Json<Value> {
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

    // if let Some(owner_uuid) = page.owner.unwrap().uuid {
    //     map.remove(&owner_uuid); // Pass a reference to the key
    // }
}