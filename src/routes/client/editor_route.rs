use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum_client_ip::InsecureClientIp;
use crate::AppState;
use crate::structs::account::Account;

pub async fn get_shared(State(app_state): State<Arc<AppState>>, Json(account): Json<Account>){

}
