use std::net::Ipv4Addr;
use std::sync::Arc;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::Json;
use axum_client_ip::InsecureClientIp;
use mongodb::bson::{doc, Uuid};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use tower_cookies::cookie::CookieJar;
use tracing::info;

use crate::AppState;
use crate::error::db_error::DbError;
use crate::structs::account::Account;
use crate::structs::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo{
    pub(crate) nickname: Option<String>,
    pub(crate) password: Option<String>
}

pub async fn register_account(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp,
                              Json(account_info): Json<AccountInfo>,
) -> Json<serde_json::Value> {
    let result = app_state.db.user_collection
        .find_one(doc! {"account.nickname" : account_info.clone().nickname }, None).await
        .expect("TODO: panic message");

    match result {
        None => {
            let uuid = Some(Uuid::new());
            let user = User::new(
                uuid.clone().unwrap().to_string(),
                Account::new(account_info.nickname, account_info.password),
                insecure_ip.0.to_string(),
            );

            let mut users = app_state.users.lock().unwrap();
            users.push(user);

            info!("asdfasdf  {:?}", users);

            Json(json!({"uuid": uuid.expect("REASON").to_string()}))
        }
        Some(_) => {
            Json(json!({"uuid": "undefined"}))
        }
    }
}

pub async fn login_account(State(app_state): State<Arc<AppState>>, Json(account_info): Json<AccountInfo>) -> Json<serde_json::Value> {
    let users_lock = app_state.users.lock().unwrap();

    for user in users_lock.iter() {
        let user_clone = user.clone();

        if  user.to_owned().account.unwrap().nickname.unwrap().eq(&account_info.nickname.to_owned().unwrap())
            && user.to_owned().account.unwrap().password.unwrap().eq(&account_info.password.to_owned().unwrap()){
            info!("{:?} Logged In successfully!", user.account.to_owned().unwrap().nickname);

            return Json(json!({"uuid": user.uuid.clone().unwrap()}))
        }
    }

    Json(json!({"uuid": "undefined"}))
}



pub async fn get_account(State(app_state): State<Arc<AppState>>, Path(user_id): Path<String>) -> Json<serde_json::Value> {
    let uuid = Uuid::parse_str(&user_id).unwrap();
    let account = app_state.db.user_doc_collection.find_one(doc! {"uuid": uuid},None).await;

    match account {
        Ok(_) => {
            info!("test");
            Json(json!({"data": account.unwrap() }))
        }
        Err(_) => {
            Json(json!({ "custom_status": "404"}))
        }
    }
}