use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use axum_client_ip::InsecureClientIp;
use serde_json::json;
use crate::AppState;

//TODO change parameter's type to BinaryUUID on every routes
pub async fn get_members(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>)  -> impl IntoResponse {
    
    Json(json!({"":""}))
}

pub async fn get_guests(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>)  -> impl IntoResponse {
    Json(json!({"":""}))
}


pub async fn invite_members(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>)  -> impl IntoResponse {
    Json(json!({"":""}))
}

pub async fn invite_guests(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>) -> impl IntoResponse {
    Json(json!({"":""}))
}

pub async fn exile_members(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>) -> impl IntoResponse {
    Json(json!({"":""}))
}
//TODO Exile users with reasons
pub async fn exile_guests(State(app_state): State<Arc<AppState>>, Path(uuid): Path<String>)  -> impl IntoResponse {

    Json(json!({"":""}))
}