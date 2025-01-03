mod types;

use reqwest::Client as ReqwestClient;
use crate::error::GeckoError;
pub use types::*;

const BASE_URL: &str = "https://api.geckoterminal.com/api/v2";

pub struct GeckoClient {
    client: ReqwestClient,
}

impl GeckoClient {
    pub fn new() -> Self {
        Self {
            client: ReqwestClient::new(),
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, GeckoError> {
        let url = format!("{}{}", BASE_URL, path);
        
        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;

        let text = response.text().await?;
        serde_json::from_str(&text).map_err(GeckoError::ParseError)
    }
}

impl Default for GeckoClient {
    fn default() -> Self {
        Self::new()
    }
}