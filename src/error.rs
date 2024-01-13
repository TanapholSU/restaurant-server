
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::extract::rejection::{JsonRejection, PathRejection};
use serde_json::json;
use sqlx::Error as DbError;
use thiserror::Error;

#[derive(Error, Debug)]
/// Custom error for server application
pub enum ApiError{
    #[error("Database error {0}")]
    DatabaseError(DbError),

    #[error("Bad request from client. Reason: {0}")]
    BadRequest(String),

    #[error("Server error. Reason: {0}")]
    ServerError(String),

    #[error("Order not found")]
    OrderNotFound,

    #[error("Table not found")]
    TableNotFound,

    #[error(transparent)]
    InvalidJsonRequest(#[from] JsonRejection),

    #[error(transparent)]
    InvalidPathRequest(#[from] PathRejection)
}

impl ApiError{

    /// utility function to return status code for each error 
    pub fn status_code(&self) -> u16{
        match self{
            ApiError::DatabaseError(_) => 500,
            ApiError::BadRequest(_) =>   400,
            ApiError::ServerError(_) => 500,
            ApiError::TableNotFound => 404,
            ApiError::OrderNotFound => 404,
            ApiError::InvalidJsonRequest(_) => 400,
            ApiError::InvalidPathRequest(_) => 400
        }
    }

    pub fn axum_status_code(&self) -> StatusCode{
        StatusCode::from_u16(self.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}


impl IntoResponse for ApiError{
    /// custom response function for ApiError
    fn into_response(self) -> axum::response::Response {
        
        let error_message = match &self{
            ApiError::DatabaseError(err) => format!("Database error -> {err}"),
            ApiError::BadRequest(err) => format!("Bad request -> {err}"),
            ApiError::ServerError(err) => format!("Server error -> {err}"),
            ApiError::TableNotFound => format!("Table not found"),
            ApiError::OrderNotFound => format!("Order not found"),
            ApiError::InvalidJsonRequest(_err) => format!("Bad request -> Json request payload is incorrect"),
            ApiError::InvalidPathRequest(_err) => format!("Bad request -> parameters in path are incorrect")
        };

        (
            self.axum_status_code(), 
            axum::extract::Json(json!({
                "status_code": self.status_code(),
                "error_cause": error_message
            }))
        ).into_response()
        
    }
}

