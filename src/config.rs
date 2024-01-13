use serde::Deserialize;

#[derive( Debug, Deserialize, Clone)]
pub struct AppConfig{
    pub db_url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub max_tables: Option<i16>
}
