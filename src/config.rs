
use envy;
use dotenvy;
use serde::Deserialize;

#[derive( Debug, Deserialize, Clone)]
pub struct AppConfig{
    pub db_url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub max_tables: Option<i16>
}


impl AppConfig{

    /// this function read env parameters and create config
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

    /// function to get copied of db_url if exists. otherwise, default testing db url is returned
    pub fn get_db_url(&self) -> String{
        self.db_url.clone().unwrap_or(r#"postgres://postgres:password@localhost/test"#.to_string())
    }

    /// function to get copied of max_tables if exists. otherwise, default testing parameter value is returned
    pub fn get_max_tables(&self) -> i16{
        self.max_tables.unwrap_or(100)
    }

    /// function to get copied of host if exists. otherwise, default testing parameter value is returned
    pub fn get_host(&self) -> String{
        self.host.clone().unwrap_or("localhost".to_string())
    }

    /// function to get copied of port if exists. otherwise, default testing parameter value is returned
    pub fn get_port(&self) -> u16{
        self.port.unwrap_or(3000)
    }
}
