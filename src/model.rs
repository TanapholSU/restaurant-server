

use axum::{response::IntoResponse, http::{StatusCode, header}};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;


/// This struct represents an order record in the database.
#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq, Eq, Clone)]
pub struct OrderItem{
    /// order_id is unique identifier for each order and the data type is i32 because postgres doesn't support unsigned.
    /// In additions, for simple restaurant, it is easier to read for user 
    pub order_id: i32,

    /// Table id of the order
    pub table_id: i16,

    /// The order name. This is because uniqueness is not in the requirements.
    /// It is possible that restaurant may provides made to order dish (like in Thailand). 
    pub item_name: String,

    ///  just a note from staff in case that customer has some  preference (e.g., not spicy)
    pub note: Option<String>,

    /// creation_time is the time that order request is processed (UTC). It can be useful for investigation when some problem happen
    pub creation_time:  DateTime<Utc>,

    /// estimated_arrival_time is the estimated arrival time (UTC)
    pub estimated_arrival_time: DateTime<Utc>
}


/// Lightweight version of OrderItem. It is used by client to construct TableOrderRequest request payload to server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItemRequest{
    pub table_id: i16,
    pub item_name: String,
    pub note: Option<String>,
}


/// A struct that contains table id and vector of order requests
/// at present, it contains redundant information of table_id
/// However, in the future, we can include more attribute like table availibity 
/// or payment status later if user wants more functionalities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableOrdersRequest{
    pub table_id: i16,
    pub orders: Vec<OrderItemRequest>

}

impl OrderItemRequest{

    /// Helper function to create OrderItemRequest struct
    pub fn new(table_id: i16, item_name:&str, note:&str) -> Self{
        OrderItemRequest{
            table_id: table_id,
            item_name: item_name.to_string(),
            note: Some(note.to_string())
        }
    }

    /// Helper function to create OrderItemRequest struct without note
    pub fn new_wihout_note(table_id: i16, item_name:&str) -> Self{
        OrderItemRequest{
            table_id: table_id,
            item_name: item_name.to_string(),
            note: None
        }
    }
}


impl TableOrdersRequest{

    /// Helper function to create table order struct
    pub fn new(table_id: i16) -> Self{
        Self { table_id: table_id, orders: Vec::new() }
    }

    /// Helper function to create new order item request and attch to  OrderItemRequest struct
    pub fn add_order(&mut self, item_name: &str, note: &str){
        self.orders.push(OrderItemRequest::new(self.table_id, item_name, note));
    }

    /// Helper function to create new order item (without note) request and attch to  OrderItemRequest struct
    pub fn add_order_wihtout_note(&mut self, item_name: &str){
        self.orders.push(OrderItemRequest::new_wihout_note(self.table_id, item_name));
    }

    /// This function consume TableOrderRequest and returns Vec<OrderItemRequest>
    /// The result is later sent to business logic to determine arrival time and then insert to actual db
    pub fn get_orders(self) -> Vec<OrderItemRequest>{
        self.orders
    }
    
    pub fn to_json(&self)->Result<String, serde_json::Error>{
        serde_json::to_string_pretty(self)
    }
}


/// This struct represents the responds payload returning back to client
/// It contains both table_id and list of orderitem
#[derive(Serialize, Deserialize, Debug)]
pub struct TableOrdersResponse{
    /// status code (for future error implementation)
    pub status_code: u16,

    /// table id of the orders
    pub table_id: i16,

    /// orders belonging to table_id
    pub orders: Vec<OrderItem>
}


impl TableOrdersResponse{
    /// Utility function for creating new TableOrdersResponse
    pub fn new(status_code: u16, table_id: i16, orders: Vec<OrderItem>) -> Self{
        Self { status_code: status_code, table_id: table_id, orders: orders }
    }
}


impl IntoResponse for TableOrdersResponse{
    /// trait implementation to convert TableOrdersResponse to Axum response
    fn into_response(self) -> axum::response::Response {
        match serde_json::to_string_pretty(&self){
            Ok(json) => {
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "application/json")],
                    json
                ).into_response()
            },
            Err(_) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(header::CONTENT_TYPE, "application/json")],
                    "{\"status_code\":500,\"error_cause\":\"json serialization error (invalid status code)\"}".to_string(),
                ).into_response()
            }
        }
    }
}
