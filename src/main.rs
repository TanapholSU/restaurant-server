use axum::{Router, routing::get};
use restaurant_server::{config::AppConfig, handlers::handle_health_check};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<AppConfig>().unwrap_or(
        AppConfig { 
            db_url: None,
            host: None, 
            port: None, 
            max_tables: None 
    });


    tracing_subscriber::fmt().with_thread_names(true).init();
    tracing::info!("{config:?}");

    println!("Hello, world!");

    let router = Router::new()
                            .route("/api/v1/health", get(handle_health_check))
                            .fallback(|| async { "hello paidy restaurant"});
    
    let listener = TcpListener::bind("127.0.0.1:3000")
                                    .await
                                    .expect(&format!("Unable to bind server"));

    tracing::info!("Server is ready");
    axum::serve(listener, router).await.expect("Cannot serve service");
}
