use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeckoResponse<T> {
    pub data: T,
}

// Price types
#[derive(Debug, Deserialize)]
pub struct TokenPrice {
    pub attributes: TokenPriceAttributes,
}

#[derive(Debug, Deserialize)]
pub struct TokenPriceAttributes {
    pub token_prices: std::collections::HashMap<String, String>,
}

// Pool types
#[derive(Debug, Deserialize)]
pub struct Pool {
    pub id: String,
    pub attributes: PoolAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]  // Silence warnings about unused fields
pub struct PoolAttributes {
    pub name: String,
    pub address: String,
    pub base_token_price_usd: Option<String>,
    pub quote_token_price_usd: Option<String>,
    pub base_token_price_quote_token: Option<String>,
    pub reserve_in_usd: Option<String>,
}