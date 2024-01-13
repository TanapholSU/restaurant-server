use restaurant_server::error::ApiError;
use restaurant_server::{ context, routes, config};
use sqlx::error::*;
use tokio::net::TcpListener;



#[tokio::main]
async fn main() -> Result<(), ApiError>{
    let config = config::AppConfig::new_from_env();
    let host = config.get_host();
    let port = config.get_port();

    let database_url =  config.get_db_url();

    tracing::info!("HOST:{host} PORT:{port}  DB URL:{database_url}");

    let context = context::ApiContext::new_from_env().await?;

    let router = routes::app(context);
    let listener = TcpListener::bind(format!("{host}:{port}")).await
                                    .expect(&format!("Unable to bind server to {host}:{port}"));

    tracing::info!("Server is ready");
    axum::serve(listener, router).await.expect("Cannot serve service");
    Ok(())
}