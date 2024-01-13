use rusty::error::ApiError;
use sqlx::error::*;


use rusty::run_server;

#[tokio::main]
async fn main() -> Result<(), ApiError>{
    tracing_subscriber::fmt().with_thread_names(true).init();
    run_server().await
}