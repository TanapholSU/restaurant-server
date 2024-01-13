use axum::Router;
use axum::routing::{get, post, delete};
use crate::context::ApiContext;
use crate::handlers::handle_health_check;


pub fn app(context: ApiContext) -> Router{
    let router = Router::new()
    .route("/api/v1/health", get(handle_health_check))
    .route("/api/v1/tables/:table_id/orders", get(handle_health_check))
    .route("/api/v1/tables/:table_id/orders",  get(handle_health_check))
    .route("/api/v1/tables/:table_id/orders/:order_id",get(handle_health_check))
    .route("/api/v1/tables/:table_id/orders/:order_id", get(handle_health_check))
    .fallback(|| async{ "hello paidy restaurant"})        
    .with_state(context);
    router
}
