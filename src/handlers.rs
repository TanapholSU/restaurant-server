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



/// helper function to extract table_id from list of orderitems
/// It returns true if table id is valid. Otherwise, false is returned
fn validate_table_id_from_orders_requests_and_path(input: &[OrderItemRequest], table_id_from_path: i16) -> bool{
    match input{
        [] => false,
        [head, tail@..] => {
            tail.iter()
                .all(|item| head.table_id == item.table_id)
                .then(||head.table_id == table_id_from_path).unwrap_or(false)
        }
    }
}



/// utility macro to check input range whether it's from 1..=max_range or not. 
/// If the input value is out of range, the error_type will be convert to Axum's response and returned to client
macro_rules! check_range {
    ($max_range: expr, $value: expr, $error_type: expr) => {
        if !(1..=$max_range).contains(&$value){
            // format!("check range failed input={} max_range={}", $value, $max_range).as_str()
            tracing::error!("out of range input value={} range=[0,{}]" , $value, $max_range);

            return $error_type.into_response();
        }
    };
}


/// validation macro for validating table id from OrderItems and id from path
macro_rules! validate_table_id_from_orders_and_path {
    ( $orders:expr, $table_id_from_path:expr)=>{
        if !validate_table_id_from_orders_requests_and_path($orders, $table_id_from_path){
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

    check_range!(context.config.get_max_tables(), table_id, ApiError::TableNotFound);
    validate_table_id_from_orders_and_path!(&table_orders.orders, table_id);

    let orders = process_order_requests(table_orders);
    tracing::info!("[add] adding orders (size= {})", orders.len());
    
    context.dbo.add_table_orders(&orders) // add orders to a table
        .and_then( |_| context.dbo.get_table_orders(table_id)) // get updated table orders
        .await
        .and_then( |orders: Vec<OrderItem>| Ok(TableOrdersResponse::new(200,  table_id,  orders).into_response()))// generate TableOrdersResponse from orders
        .unwrap_or_else(ApiError::into_response)  // generate error response in case of error
}


/// handler function for getting all table's order (of a specific table). 
pub async fn handle_get_all_orders_for_specific_table(
        State(context): State<ApiContext>, 
        WithRejection(Path(table_id), _): WithRejection<Path<i16>, ApiError>) ->  Response{

    tracing::info!("[get all] table id from path = {table_id}");
    check_range!(context.config.get_max_tables(), table_id, ApiError::TableNotFound);

    context.dbo.get_table_orders(table_id) // get tables order
        .await
        .and_then( |orders: Vec<OrderItem>| Ok(TableOrdersResponse::new(200,  table_id,  orders).into_response()))// generate TableOrdersResponse from orders
        .unwrap_or_else(ApiError::into_response) // generate error response in case of error
}


//handler function for getting specific order 
pub async fn handle_get_specific_table_order(State(context): State<ApiContext>, 
    WithRejection(Path((table_id, order_id)), _): WithRejection<Path<(i16, i32)>, ApiError>)  ->  Response{
    
    tracing::info!("[get specific] table id = {table_id}, order_id= {order_id} from path");
    
    check_range!(context.config.get_max_tables(), table_id, ApiError::TableNotFound);
    check_range!(i32::MAX, order_id, ApiError::OrderNotFound);


    context.dbo.get_specific_table_order(table_id, order_id) // get specific order
        .await
        .and_then( |orders: Vec<OrderItem>| Ok(TableOrdersResponse::new(200,  table_id,  orders).into_response()))// generate TableOrdersResponse from orders
        .unwrap_or_else(ApiError::into_response) // generate error response 
}


//handler function for delete a specific table's order (of a specific table). Then returns the updated table's orders (TableOrderResponse)
pub async fn handle_delete_table_order(State(context): State<ApiContext>, 
    WithRejection(Path((table_id, order_id)), _): WithRejection<Path<(i16, i32)>, ApiError>) ->  impl IntoResponse{
    
    tracing::info!("[delete] table id = {table_id}, order_id= {order_id} from path");
    

    check_range!(context.config.get_max_tables(), table_id, ApiError::TableNotFound);
    check_range!(i32::MAX, order_id, ApiError::OrderNotFound);

    // validate_table_id_range!(context, table_id);    
    // validate_order_id_range!(order_id);
    
    context.dbo.remove_order(table_id, order_id) // remove order
        .and_then( |_| context.dbo.get_table_orders(table_id)) // get updated table orders
        .await
        .and_then( |orders: Vec<OrderItem>| Ok(TableOrdersResponse::new(200,  table_id,  orders).into_response()))// generate TableOrdersResponse from orders
        .unwrap_or_else(ApiError::into_response) // generate error response in case of error
}



#[cfg(test)]
mod test{
    use crate::model::OrderItemRequest;

    use super::*;
    #[test]
    fn test_process_one_order_request(){
        let current_time = Utc::now();
        let order_request = OrderItemRequest::new(1, "A", "B");

        let result = process_order_request(order_request, current_time);

        assert_eq!(result.order_id, -1);
        assert_eq!(result.table_id, 1);
        assert_eq!(result.item_name, "A");
        assert_eq!(result.note, Some("B".to_string()));
        
        let diff = result.estimated_arrival_time - result.creation_time;
        assert!(diff.num_minutes() >= 5);
        assert!(diff.num_minutes() <= 15);
    }

    #[test]
    fn test_process_order_requests(){

        let mut orders = TableOrdersRequest::new(1);

        orders.orders = vec![
            OrderItemRequest::new(1, "A", "B"),
            OrderItemRequest::new(1, "C", "D")
        ];

        let results = process_order_requests(orders);

        assert_eq!(results.len(), 2);

        let result = &results[0];
        assert_eq!(result.order_id, -1);
        assert_eq!(result.table_id, 1);
        assert_eq!(result.item_name, "A");
        assert_eq!(result.note, Some("B".to_string()));
        
        let diff = result.estimated_arrival_time - result.creation_time;
        assert!(diff.num_minutes() >= 5);
        assert!(diff.num_minutes() <= 15);

        let result = &results[1];
        assert_eq!(result.order_id, -1);
        assert_eq!(result.table_id, 1);
        assert_eq!(result.item_name, "C");
        assert_eq!(result.note, Some("D".to_string()));
        
        let diff = result.estimated_arrival_time - result.creation_time;
        assert!(diff.num_minutes() >= 5);
        assert!(diff.num_minutes() <= 15);
    }


    #[test]
    fn test_validate_table_id_from_orders(){
        let order1 = OrderItemRequest::new_wihout_note(1, "A");
        let order2 = OrderItemRequest::new_wihout_note(1, "B");
        let order3 = OrderItemRequest::new_wihout_note(2, "B");

        assert_eq!(validate_table_id_from_orders_requests_and_path(&vec![order1.clone(), order2.clone()], 1), true);
        assert_eq!(validate_table_id_from_orders_requests_and_path(&vec![order1.clone(), order2.clone()], 5), false);
        assert_eq!(validate_table_id_from_orders_requests_and_path(&vec![order1.clone(), order3.clone()], 1), false);
    }

    
}
