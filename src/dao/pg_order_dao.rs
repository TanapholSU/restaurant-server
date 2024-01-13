use sqlx::PgPool;

use crate::{model::OrderItem, error::ApiError};

use super::order_dao::TableOrderDAO;


#[derive(Clone)]
/// Implementation of order DAO
pub struct PgTableOrderDAO{
    pub db: PgPool
}


impl PgTableOrderDAO{
    
    /// utility function for get specific order. It returns ApiError::OrderNotFound if returning result is 0. Otherwise, OK with query result
    fn is_existing_order(orders: Vec<OrderItem>) -> Result<Vec<OrderItem>, ApiError>{
        if orders.len() == 0{
            Err(ApiError::OrderNotFound)
        }else{
            Ok(orders)
        }
    }

}

/// utility macro function to produce closure for converting sqlx error to ApiError (custom error)
macro_rules! sqlx_error_to_api_error {
    ($error_message: expr) => {
        |err| ApiError::DatabaseError( err)
    };
}



impl TableOrderDAO for PgTableOrderDAO{
    #[doc = r" function for adding OrderItems to table (each item already contains table_id)"]
    async fn add_table_orders(&self,items: &[OrderItem]) -> Result<(),ApiError>  {
        todo!()
    }

    #[doc = r" function to get all OrderItems for specific table_id"]
    async fn get_table_orders(&self, table_id: i16) -> Result<Vec<OrderItem>, ApiError> {
        sqlx::query_as("SELECT * FROM ORDERS WHERE table_id = $1 ORDER BY order_id")
                .bind(table_id)
                .fetch_all(&self.db)
                .await
                .map_err(sqlx_error_to_api_error!("Could not query table orders from DB"))
    }

    #[doc = r" function to get specific OrderItem (in a vec for simplicity for caller) in the specific table"]
    async fn get_specific_table_order(&self, table_id: i16, order_id: i32) -> Result<Vec<OrderItem>, ApiError>{
        sqlx::query_as("SELECT * FROM ORDERS WHERE table_id = $1 and order_id = $2 LIMIT 1")
                .bind(table_id)
                .bind(order_id)
                .fetch_all(&self.db).await
                .map_err(sqlx_error_to_api_error!("Could not query specific table order from DB"))
                .and_then(PgTableOrderDAO::is_existing_order)
    }

    #[doc = r" function to remove specific OrderItem from DB"]
    async fn remove_order(&self,table_id:i16,order_id:i32) -> Result<(),ApiError>  {
        todo!()
    }


}

    