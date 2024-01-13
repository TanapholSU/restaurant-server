use std::sync::Arc;
use crate::config::AppConfig;

/// struct to store configuration as shared state in axum.
#[derive(Clone)]
pub struct ApiContext {
    /// variable to store config parameters from environments (using hashmap for future proof)
    pub config: Arc<AppConfig>,

}

