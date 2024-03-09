use std::sync::Arc;
use axum::extract::State;
use axum::{Json, response::IntoResponse};
use axum_client_ip::InsecureClientIp;
use async_std::sync::RwLock;
use redis::{AsyncCommands, Commands};
use crate::AppState;

pub async fn test_post_handler(State(app_state): State<Arc<AppState>>, _: InsecureClientIp) -> impl IntoResponse {
    let app_state_clone = app_state.clone(); // Clone the Arc<AppState>

    // let mut lock = connection.write().await.unwrap();
    // lock.set::<&str, &str, ()>("test", "1234").await.unwrap();
    Json(serde_json::json!({"data": "" }))
}
