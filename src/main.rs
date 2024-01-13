use restaurant_server::error::ApiError;
use sqlx::error::*;


use restaurant_server::run_server;

#[tokio::main]
async fn main() -> Result<(), ApiError>{
    tracing_subscriber::fmt().with_thread_names(true).init();
    run_server().await
}