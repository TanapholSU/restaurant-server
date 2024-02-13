
use std::sync::Arc;
use sqlx::{self, postgres::PgPoolOptions};
use crate::{dao::pg_order_dao::PgTableOrderDAO, config::AppConfig, error::ApiError};


/// struct to store configuration as shared state in axum.
#[derive(Clone)]
pub struct ApiContext {
    /// variable to store config parameters from environments (using hashmap for future proof)
    pub config: Arc<AppConfig>,

    /// official async support in rust 1.75 doesn't support dyn trait object yet. So it is fixed to postgres impl for now 
    pub dbo: PgTableOrderDAO  
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
        .max_connections((&config).get_max_db_pool_size())
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
