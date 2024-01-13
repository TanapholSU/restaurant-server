
use trait_variant;
use crate::model::OrderItem;
use crate::error::ApiError;

/// trait for database access object. contains add / get / remove order record(s) functions 
#[trait_variant::make(HttpService: Send)]
pub trait TableOrderDAO{
    /// function for adding OrderItems to table (each item already contains table_id)
    async fn add_table_orders(&self, items: &[OrderItem]) ->  Result<(), ApiError> ;

    /// function to get all OrderItems for specific table_id
    async fn get_table_orders(&self, table_id: i16) -> Result<Vec<OrderItem>, ApiError>;

    /// function to get specific OrderItem (in a vec for simplicity for caller) in the specific table
    async fn get_specific_table_order(&self, table_id: i16, order_id: i32) -> Result<Vec<OrderItem>, ApiError>;

    /// function to remove specific OrderItem from DB
    async fn remove_order(&self, table_id: i16, order_id: i32) -> Result<(), ApiError>;
}

