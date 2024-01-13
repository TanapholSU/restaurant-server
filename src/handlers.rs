use axum::{http::StatusCode, Json};
use serde_json::{Value, json};

pub async fn handle_health_check() -> (StatusCode, Json<Value>){
    ( 
        StatusCode::OK,
        json!({
            "status": "healthy!"
        }).into()
    )
}