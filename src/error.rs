use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeckoError {
    #[error("API Error: {0}")]
    ApiError(String),
    
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Parsing Error: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Invalid Pool Data: {0}")]
    InvalidPoolData(String),
}