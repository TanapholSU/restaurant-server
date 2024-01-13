use axum::Json;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::WithRejection;
use serde_json::{json, Value};
use futures::TryFutureExt;

use crate::dao::order_dao::TableOrderDAO;
use crate::error::ApiError;
use crate::model::{TableOrdersRequest, TableOrdersResponse, OrderItem, OrderItemRequest};
use crate::context::ApiContext;
use chrono::{DateTime,Duration, Utc};


use rand::{thread_rng, Rng};

/// Internal function to convert OrderItemRequest to order item. 
/// During transformation, it assign estimated arrival time for each order
fn process_order_request(order_request:  OrderItemRequest, current_time: DateTime<Utc>) -> OrderItem{
    let estimated_time = current_time + Duration::minutes(thread_rng().gen_range(5..=15));

    OrderItem{
        order_id: -1,
        table_id: order_request.table_id,
        item_name: order_request.item_name,
        note: order_request.note.clone(),
        creation_time: current_time.clone(),
        estimated_arrival_time: estimated_time,
    }
}


/// Function to estimate order arrival time, and conver OrderItemRequests to OrderItem for inserting to DB
/// The reason that it consumes Vec instead of reference ($[OrderItemRequest]) is because there is no need to used OrderItemRequest object anymore after calling this function
pub fn process_order_requests(order_request:  TableOrdersRequest) -> Vec<OrderItem>{
    let current_time: DateTime<Utc> = Utc::now();
    order_request.orders.into_iter().map(|order_request| process_order_request(order_request, current_time)).collect()
}



/// utility macro to use dbo to execute query for specific table with table_id
macro_rules! get_table_orders {
    ($context:expr,$table_id:expr)=>{
        |_| $context.dbo.get_table_orders($table_id)
    };
}

/// utility macro to convert all Vec<OrderItems> to TableOrderResponse. Then, the result object is converted to Axum Response later
macro_rules! convert_order_items_to_table_order_response {
    ( $table_id:expr)=>{
        |orders: Vec<OrderItem>| Ok(TableOrdersResponse::new(200,  $table_id,  orders).into_response())
    };
}

/// utility function for validating whether the input table_id is in acceptable range or not
/// It returns true if table id is valid. Otherwise, false is returned
fn validate_table_id_range(max_tables:i16, table_id: i16) -> bool{
    if (1..=max_tables).contains(&table_id){
        return true;
    }
    return false
}

/// helper function to extract table_id from list of orderitems
/// It returns true if table id is valid. Otherwise, false is returned
fn validate_table_id_from_orders_requests(input: &[OrderItemRequest], table_id_from_path: i16) -> bool{
    match input{
        [] => false,
        [head, tail@..] => {
            tail.iter()
                .all(|item| head.table_id == item.table_id)
                .then(||head.table_id == table_id_from_path).unwrap_or(false)
        }
    }
}


/// validation macro for checking table id 
macro_rules! validate_table_id_from_path {
    ( $context:expr, $table_id:expr)=>{
        if !validate_table_id_range($context.config.get_max_tables(), $table_id){
            return ApiError::TableNotFound.into_response();
        }
    };
}


/// validation macro for validating table id from OrderItems and id from path
macro_rules! validate_table_id_from_orders {
    ( $orders:expr, $table_id_from_path:expr)=>{
        if !validate_table_id_from_orders_requests($orders, $table_id_from_path){
            return ApiError::BadRequest(format!("table id in json request (or path) is incorrect")).into_response();
        }
    };
}


/// handler function for health check operation which checks the db whether it is alive or not 
pub async fn handle_health_check(State(context): State<ApiContext>) ->  (axum::http::StatusCode, Json<Value>){
    
    tracing::info!("[health check]");

    match sqlx::query("SELECT 1").execute(&context.dbo.db).await{
        Ok(_) => {
            (
                StatusCode::OK, 
                json!(
                    {
                        "status": "healthy!"
                    }
                ).into()
            )
        },
        Err(err) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                json!(
                    {
                    "status": format!("Database error {err}")
                    }
                ).into()
            )
        }
    }

    
}


/// handler function for processing incoming TableOrderRequests,  insert new orders to database, and then returns the updated table's orders (TableOrderResponse)
pub async fn handle_add_orders(State(context): State<ApiContext>, 
                        WithRejection(Path(table_id), _): WithRejection<Path<i16>, ApiError>, 
                        WithRejection(Json(table_orders), _): WithRejection<Json<TableOrdersRequest>, ApiError>) 
                        ->  Response{
    
    tracing::info!("[add] table id from path = {}, max_table = {}", table_id, context.config.get_max_tables());

    validate_table_id_from_path!(context, table_id);
    tracing::info!("[add] check table id from path -> OK");
    validate_table_id_from_orders!(&table_orders.orders, table_id);
    tracing::info!("[add] check table id from orders -> OK");
                        
    let orders = process_order_requests(table_orders);
    tracing::info!("[add] adding orders (size= {})", orders.len());

    context.dbo.add_table_orders(&orders) // add orders to a table
        .and_then( |_| context.dbo.get_table_orders(table_id)) // chain get updated table orders future
        .await
        .and_then(convert_order_items_to_table_order_response!(table_id))// generate TableOrdersResponse from orders
        .unwrap_or_else(ApiError::into_response)  // generate error response in case of error
}


/// handler function for getting all table's order (of a specific table). 
pub async fn handle_get_all_orders_for_specific_table(
        State(context): State<ApiContext>, 
        WithRejection(Path(table_id), _): WithRejection<Path<i16>, ApiError>) ->  Response{

    tracing::info!("[get all] table id from path = {table_id}");
    validate_table_id_from_path!(context, table_id);
    
    tracing::info!("[get all] getting table orders from table {table_id}");
    context.dbo.get_table_orders(table_id) // get tables order
        .await
        .and_then(convert_order_items_to_table_order_response!(table_id))// convert table orders to TableOrdersResponse
        .unwrap_or_else(ApiError::into_response) // generate error response in case of error
}


//handler function for getting specific order 
pub async fn handle_get_specific_table_order(State(context): State<ApiContext>, 
    WithRejection(Path((table_id, order_id)), _): WithRejection<Path<(i16, i32)>, ApiError>)  ->  Response{
    
    tracing::info!("[get specific] table id = {table_id}, order_id= {order_id} from path");
    validate_table_id_from_path!(context, table_id);

    tracing::info!("[get specific] getting specific order from table {table_id} -> {order_id}");
    context.dbo.get_specific_table_order(table_id, order_id) // get specific order
        .await
        .and_then(convert_order_items_to_table_order_response!(table_id))// if success, convert to TableOrderResponse
        .unwrap_or_else(ApiError::into_response) // generate error response 
}


//handler function for delete a specific table's order (of a specific table). Then returns the updated table's orders (TableOrderResponse)
pub async fn handle_delete_table_order(State(context): State<ApiContext>, 
    WithRejection(Path((table_id, order_id)), _): WithRejection<Path<(i16, i32)>, ApiError>) ->  impl IntoResponse{
    
    tracing::info!("[delete] table id = {table_id}, order_id= {order_id} from path");
    validate_table_id_from_path!(context, table_id);

    tracing::info!("[delete] deleting order from table {table_id} -> {order_id}");
    context.dbo.remove_order(table_id, order_id) // remove order
        .and_then(get_table_orders!(context, table_id))// get updated orders for the table
        .await
        .and_then(convert_order_items_to_table_order_response!(table_id)) // convert order item to TableOrdersResponse
        .unwrap_or_else(ApiError::into_response) // generate error response in case of error
}

