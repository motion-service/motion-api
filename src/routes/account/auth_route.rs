use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::{StatusCode};
use axum::Json;
use axum_client_ip::InsecureClientIp;
use mongodb::bson::{doc, Uuid};
use serde_json::{json, Value};

use crate::AppState;
use crate::repository::account_repository::AccountRepository;
use crate::structs::account::{Account};
use crate::structs::user::User;

pub async fn register_account(State(app_state): State<Arc<AppState>>, insecure_ip: InsecureClientIp, Json(account_info): Json<Account>) -> Json<Value> {
    let result_name = account_info.clone().name;
    let result_password = account_info.clone().password;
    // let result_password = account_info.clone().password;

    let account = Account::new(result_name, result_password);
    let existed_user = app_state.db.is_already_existed_account(account.clone()).await;

    match existed_user {
        None => {
            let user = User::new(
                account,
                insecure_ip.0.to_string(),
            );

            let id = user.id.to_string();

            app_state.db.register_account(user.to_owned()).await;
            Json(json!({"result": id}))
        }

        Some(_) => {
            Json(json!({"result": "ALREADY_EXIST_ACCOUNT"}))
        }
    }
}

pub async fn login_account(State(app_state): State<Arc<AppState>>, Json(account_info): Json<Account>) -> Json<Value> {
    let result_name = account_info.clone().name;
    let result_password = account_info.clone().password;

    let account = Account::new(result_name, result_password);
    let user = app_state.db.is_already_existed_account(account).await;

    if user.is_some(){
        return Json(json!({"uuid": user.unwrap().id.to_string()}))
    }

    Json(json!({"" : ""}))
}

pub async fn get_account(State(app_state): State<Arc<AppState>>, Path(user_id): Path<String>) -> Json<Value> {
    let uuid = Uuid::parse_str(&user_id).unwrap();
    let account = app_state.db.user_doc_collection.find_one(doc! {"uuid": uuid},None).await;

    match account {
        Ok(_) => {

            Json(json!({"data": account.unwrap() }))
        }
        Err(_) => {
            Json(json!({ "custom_status": "404"}))
        }
    }
}