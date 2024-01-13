use sqlx::PgPool;

use super::order_dao::TableOrderDAO;


#[derive(Clone)]
/// Implementation of order DAO
pub struct PgTableOrderDAO{
    pub db: PgPool
}


impl TableOrderDAO for PgTableOrderDAO{
    #[doc = r" function for adding OrderItems to table (each item already contains table_id)"]
    async fn add_table_orders(&self,items: &[OrderItem]) -> Result<(),ApiError>  {
        todo!()
    }

    #[doc = r" function to get all OrderItems for specific table_id"]
    async fn get_table_orders(&self,table_id:i16) -> Result<Vec<OrderItem> ,ApiError>  {
        todo!()
    }

    #[doc = r" function to get specific OrderItem (in a vec for simplicity for caller) in the specific table"]
    async fn get_specific_table_order(&self,table_id:i16,order_id:i32) -> Result<Vec<OrderItem> ,ApiError>  {
        todo!()
    }

    #[doc = r" function to remove specific OrderItem from DB"]
    async fn remove_order(&self,table_id:i16,order_id:i32) -> Result<(),ApiError>  {
        todo!()
    }
}