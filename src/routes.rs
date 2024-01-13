use axum::Router;
use axum::routing::{get, post, delete};
use crate::context::ApiContext;
use crate::handlers::{handle_health_check, handle_add_orders, handle_get_all_orders_for_specific_table, handle_get_specific_table_order, handle_delete_table_order};


pub fn app(context: ApiContext) -> Router{
    let router = Router::new()
    .route("/api/v1/health", get(handle_health_check))
    .route("/api/v1/tables/:table_id/orders", post(handle_add_orders))
    .route("/api/v1/tables/:table_id/orders",  get(handle_get_all_orders_for_specific_table))
    .route("/api/v1/tables/:table_id/orders/:order_id", get(handle_get_specific_table_order))
    .route("/api/v1/tables/:table_id/orders/:order_id", delete(handle_delete_table_order))
    .fallback(|| async{ "hello paidy restaurant"})        
    .with_state(context);
    router
}
