use serde::Deserialize;

#[derive( Debug, Deserialize, Clone)]
pub struct AppConfig{
    pub db_url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub max_tables: Option<i16>
}


impl AppConfig{

    pub fn new_from_env() -> Self{
        dotenvy::dotenv().ok();
        envy::from_env::<Self>().unwrap_or(
            Self { 
                db_url: None,
                host: None, 
                port: None, 
                max_tables: None 
        })
    }

    pub fn get_db_url(&self) -> String{
        self.db_url.clone().unwrap_or(r#"postgres://postgres:password@localhost/test"#.to_string())
    }

    pub fn get_max_table(&self) -> i16{
        self.max_tables.unwrap_or(100)
    }

    pub fn get_host(&self) -> String{
        self.host.clone().unwrap_or("localhost".to_string())
    }

    pub fn get_port(&self) -> u16{
        self.port.unwrap_or(3000)
    }
}

