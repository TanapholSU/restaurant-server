
use envy;
use dotenvy;
use serde::Deserialize;

#[derive( Debug, Deserialize, Clone)]
pub struct AppConfig{
    pub database_url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub max_tables: Option<i16>,
    pub max_db_pool_size: Option<u32>
}


impl AppConfig{

    /// this function read env parameters and create config
    pub fn new_from_env() -> Self{
        dotenvy::dotenv().ok();

        envy::from_env::<Self>().unwrap_or(
            Self { 
                database_url: None,
                host: None, 
                port: None, 
                max_tables: None,
                max_db_pool_size: None
        })
    }

    /// function to get copied of db_url if exists. otherwise, default testing db url is returned
    pub fn get_db_url(&self) -> String{
        self.database_url.clone().unwrap_or(r#"postgres://postgres:password@localhost/test"#.to_string())
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

    /// function to get db connection pool size
    pub fn get_max_db_pool_size(&self) -> u32{
        self.max_db_pool_size.unwrap_or(10)
    }
}


#[cfg(test)]
mod test{
    use super::AppConfig;

    #[test]
    fn test_get_values_from_config(){
        let config = AppConfig{
            database_url: Some("URL".to_string()),
            host: Some("host".to_string()),
            port: Some(3333),
            max_tables: Some(101),
            max_db_pool_size: Some(22)
        };

        assert_eq!(config.database_url, Some("URL".to_string()));
        assert_eq!(config.host, Some("host".to_string()));
        assert_eq!(config.port, Some(3333));
        assert_eq!(config.max_tables, Some(101));
        assert_eq!(config.max_db_pool_size, Some(22));

        assert_eq!(config.get_db_url(), "URL");
        assert_eq!(config.get_host(), "host");
        assert_eq!(config.get_port(), 3333);
        assert_eq!(config.get_max_db_pool_size(), 22);

    }

    
    #[test]
    fn test_get_default_values_from_config(){
        let config = AppConfig{
            database_url: None,
            host: None,
            port: None,
            max_tables: None,
            max_db_pool_size: None
        };

        assert_eq!(config.database_url, None);
        assert_eq!(config.host,None);
        assert_eq!(config.port,None);
        assert_eq!(config.max_tables,None);
        assert_eq!(config.max_db_pool_size, None);


        assert_eq!(config.get_db_url(), r#"postgres://postgres:password@localhost/test"#);
        assert_eq!(config.get_host(), "localhost");
        assert_eq!(config.get_port(), 3000);
        assert_eq!(config.get_max_tables(), 100);
        assert_eq!(config.get_max_db_pool_size(), 10);

    }
}
