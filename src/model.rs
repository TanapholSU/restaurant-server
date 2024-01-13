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


/// This struct represents the responds payload returning back to client
/// It contains both table_id and list of orderitem
#[derive(Serialize, Deserialize, Debug)]
pub struct TableOrdersResponse{
    /// status code (for future extension)
    pub status_code: u16,

    /// table id of the orders
    pub table_id: i16,

    /// orders belonging to table_id
    pub orders: Vec<OrderItem>
}

