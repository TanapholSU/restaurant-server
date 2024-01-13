use serde_json::json;
use sqlx::Error as DbError;
use thiserror::Error;


/// Custom error for server application
#[derive(Error, Debug)]
pub enum ApiError{
    #[error("Database error {0}")]
    DatabaseError(DbError),

    #[error("Order not found")]
    OrderNotFound,

    #[error("Table not found")]
    TableNotFound,

}
