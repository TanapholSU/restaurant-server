
use axum::{Router, routing::get};
use chrono::Utc;
use restaurant_server::{config::AppConfig, handlers::handle_health_check, error::ApiError, dao::{pg_order_dao::PgTableOrderDAO, order_dao::TableOrderDAO}, model::{TableOrdersRequest, OrderItem, OrderItemRequest}, routes::app, context::ApiContext};
use sqlx::postgres::PgPoolOptions;
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


    let context = ApiContext::new_from_env().await.unwrap();
    
    let listener = TcpListener::bind("127.0.0.1:3000")
                                    .await
                                    .expect(&format!("Unable to bind server"));

    tracing::info!("Server is ready");
    axum::serve(listener, app(context)).await.expect("Cannot serve service");

    // let pool = PgPoolOptions::new()
    //     // .max_connections(10)
    //     .connect(&config.get_db_url()).await
    //     .map_err(|x| ApiError::DatabaseError(x)).unwrap();

    // let dao = PgTableOrderDAO{ db: pool };

    // let t = TableOrdersRequest{ table_id: 1, orders: vec![
    //     OrderItemRequest{ table_id: 1, item_name: format!("A"), note: Some(format!("B")) },
    //     OrderItemRequest{ table_id: 1, item_name: format!("C"), note: None }
    // ] };
    
    // let v = vec![
    //     OrderItem{table_id:1,item_name:format!("A"),note:Some(format!("B")), order_id: -1, creation_time: Utc::now(), estimated_arrival_time: Utc::now() },
    //     OrderItem{table_id:1,item_name:format!("A1"),note:Some(format!("B1")), order_id: -1, creation_time: Utc::now(), estimated_arrival_time: Utc::now() }, 
    // ];

    // let r = dao.add_table_orders( v.as_slice() ).await.unwrap();
    // let r = dao.get_table_orders( 1 ).await.unwrap();
    

    println!("{:?}", r);
        
    
}
