use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum_client_ip::InsecureClientIp;
use serde_json::{json, Value};
use crate::AppState;
use crate::structs::account::Account;

pub async fn create_block(State(app_state): State<Arc<AppState>>, Json(account): Json<Account>) -> impl IntoResponse {
    Json(json!({"":""}))
}
pub async fn delete_block(State(app_state): State<Arc<AppState>>, Json(account): Json<Account>) -> impl IntoResponse {
    Json(json!({"":""}))
}

pub async fn edit_block(State(app_state): State<Arc<AppState>>, Json(account): Json<Account>) -> impl IntoResponse {
    Json(json!({"":""}))
}

pub async fn load_block(State(app_state): State<Arc<AppState>>, Json(account): Json<Account>) -> impl IntoResponse {
    Json(json!({"":""}))
}
