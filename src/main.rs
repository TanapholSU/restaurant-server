use restaurant_server::config::AppConfig;

fn main() {
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
}
