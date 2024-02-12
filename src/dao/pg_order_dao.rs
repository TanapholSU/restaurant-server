use crate::dao::order_dao::TableOrderDAO;
use crate::model::OrderItem;
use crate::error::ApiError;
use sqlx::{Postgres, Transaction, PgPool};

#[derive(Clone)]
/// Implementation of order DAO
pub struct PgTableOrderDAO{
    pub db: PgPool
}


/// utility macro function to produce closure for converting sqlx error to ApiError (custom error)
macro_rules! sqlx_error_to_api_error {
    ($error_message: expr) => {
        |err: sqlx::Error|
            match err{
                sqlx::Error::RowNotFound => ApiError::OrderNotFound,
                _ => ApiError::DatabaseError(err),
            }
    };
}


impl PgTableOrderDAO{
    /// constructor to attach sqlx postgres pool (clonable) 
    pub fn new(db: PgPool) -> Self{
        Self{
            db
        }
    }

    /// helper function to build, and execute insert orders query (using bulk insert for performance but can be changed) 
    async fn execute_insert_orders(&self, transaction: &mut Transaction<'static, Postgres>, items: &[OrderItem]) -> Result<(), ApiError>{
        // build bulk insert query (for performance)
        let mut query_builder: sqlx::QueryBuilder<'_, Postgres> = sqlx::QueryBuilder::new("INSERT INTO orders(table_id, item_name, note, creation_time, estimated_arrival_time) ");
        query_builder.push_values(items, |mut binder, order| {
            binder.push_bind(&order.table_id)
                .push_bind(&order.item_name)
                .push_bind(&order.note)
                .push_bind(&order.creation_time)
                .push_bind(&order.estimated_arrival_time);
        });

        query_builder.build().execute(&mut **transaction)
            .await
            .map_err(sqlx_error_to_api_error!("Could not insert orders to DB"))
            // .map_err(|err| ApiError::DatabaseError(format!("Couldn't insert orders to DB: {err} ")))
            .and_then(|_| Ok(()))
    }

    
    /// utility function for get specific order. It returns ApiError::OrderNotFound if returning result is 0. Otherwise, OK with query result
    fn is_existing_order(orders: Vec<OrderItem>) -> Result<Vec<OrderItem>, ApiError>{
        if orders.len() == 0{
            Err(ApiError::OrderNotFound)
        }else{
            Ok(orders)
        }
    }

}

impl TableOrderDAO for PgTableOrderDAO{
    async fn add_table_orders(&self, items: &[OrderItem]) -> Result<(), ApiError> {
        
        // acquire transaction
        // we can  chain with the following statement but the code will be messier to my liking
        let mut transaction = self.db.begin()
            .await
            .map_err(sqlx_error_to_api_error!("Could not open db transaction"))?;
            
        self.execute_insert_orders(&mut transaction, items).await
            .and( 
                transaction.commit().await
                .map_err(sqlx_error_to_api_error!("Could not commit db transaction"))  // commit transaction
            )
    }


    async fn get_table_orders(&self, table_id: i16) -> Result<Vec<OrderItem>, ApiError> {
        
        sqlx::query_as("SELECT * FROM ORDERS WHERE table_id = $1 ORDER BY order_id")
                .bind(table_id)
                .fetch_all(&self.db)
                .await
                .map_err(sqlx_error_to_api_error!("Could not query table orders from DB"))
    }


    async fn get_specific_table_order(&self, table_id: i16, order_id: i32) -> Result<Vec<OrderItem>, ApiError>{
         
        sqlx::query_as("SELECT * FROM ORDERS WHERE table_id = $1 and order_id = $2 LIMIT 1")
                .bind(table_id)
                .bind(order_id)
                .fetch_all(&self.db).await
                .map_err(sqlx_error_to_api_error!("Could not get query specific table order from DB"))
                .and_then(PgTableOrderDAO::is_existing_order)

    }
    

    async fn remove_order(&self, table_id: i16, order_id: i32) -> Result<(), ApiError> {
        let mut transaction = self.db.begin()
            .await
            .map_err(sqlx_error_to_api_error!("Could not open db transaction"))?;

        
        sqlx::query_as( "DELETE FROM ORDERS WHERE table_id = $1 and order_id = $2 RETURNING *")
            .bind(table_id)
            .bind(order_id)
            .fetch_all(&self.db).await
            .map_err(sqlx_error_to_api_error!("Could not delete order from DB"))
            .and_then(PgTableOrderDAO::is_existing_order)
            .and(
                transaction.commit().await
                    .map_err(sqlx_error_to_api_error!("Could not close db transaction"))  // commit transaction
            )

    }

}



