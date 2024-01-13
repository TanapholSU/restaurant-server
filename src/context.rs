use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

use crate::{config::AppConfig, dao::pg_order_dao::PgTableOrderDAO, error::ApiError};

/// struct to store configuration as shared state in axum.
#[derive(Clone)]
pub struct ApiContext {
    /// variable to store config parameters from environments (using hashmap for future proof)
    pub config: Arc<AppConfig>,
    dbo: PgTableOrderDAO,
}



impl ApiContext{

    /// Helper function to generate API context object
    pub fn new(db: sqlx::PgPool) -> Self{
        let config = AppConfig::new_from_env();
        Self{
            config: Arc::new(config),
            dbo: PgTableOrderDAO::new(db)
        }
    }


    /// Helper function to read env parameters and generate context
    pub async fn new_from_env() -> Result<Self, ApiError>{
        let config = AppConfig::new_from_env();
    
        PgPoolOptions::new()
        // .max_connections(10)
        .connect(&config.get_db_url()).await
        .map_err(|x| ApiError::DatabaseError(x))
        .and_then(|pool|{
            Ok(
                Self{
                    config: Arc::new(config),
                    dbo: PgTableOrderDAO::new(pool)
                }
            )
        })
        
    }
}
