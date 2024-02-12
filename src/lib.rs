pub mod model;
pub mod error;
pub mod dao;
pub mod context;
pub mod handlers;
pub mod routes;
pub mod config;

use tokio::net::TcpListener;



pub async fn run_server() -> Result<(), error::ApiError>{
    let config = config::AppConfig::new_from_env();
    let host = config.get_host();
    let port = config.get_port();

    let database_url =  config.get_db_url();

    tracing::info!("HOST:{host} PORT:{port}");

    let context = context::ApiContext::new_from_env().await?;

    let router = routes::app(context);
    let listener = TcpListener::bind(format!("{host}:{port}")).await
                                    .expect(&format!("Unable to bind server to {host}:{port}"));

    tracing::info!("Server is ready");
    axum::serve(listener, router).await.expect("Cannot serve service");
    Ok(())

}