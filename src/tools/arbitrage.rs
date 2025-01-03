use serde::{Deserialize, Serialize};
use serde_json::json;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use crate::{
    client::{GeckoClient, GeckoResponse, Pool},
    error::GeckoError,
};

#[derive(Deserialize, Serialize)]
pub struct ArbitrageArgs {
    network: String,
    token_address: String,
    min_difference_percentage: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ArbitrageOpportunity {
    pool1_name: String,
    pool1_address: String,
    pool2_name: String,
    pool2_address: String,
    pool1_price: f64,
    pool2_price: f64,
    difference_percentage: f64,
    potential_profit_percentage: f64,
}

pub struct ArbitrageTool {
    client: GeckoClient,
}

impl ArbitrageTool {
    pub fn new() -> Self {
        Self {
            client: GeckoClient::new(),
        }
    }
}

impl Default for ArbitrageTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for ArbitrageTool {
    const NAME: &'static str = "find_arbitrage";
    type Error = GeckoError;
    type Args = ArbitrageArgs;
    type Output = Vec<ArbitrageOpportunity>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Find arbitrage opportunities for a token across different pools".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "network": {
                        "type": "string",
                        "description": "Network ID (e.g., 'eth', 'bsc')"
                    },
                    "token_address": {
                        "type": "string",
                        "description": "Token address to check for arbitrage"
                    },
                    "min_difference_percentage": {
                        "type": "number",
                        "description": "Minimum price difference percentage to consider (default: 1.0)",
                        "minimum": 0.1,
                        "default": 1.0
                    }
                },
                "required": ["network", "token_address"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = format!(
            "/networks/{}/tokens/{}/pools",
            args.network,
            args.token_address
        );
        
        let response: GeckoResponse<Vec<Pool>> = self.client.get(&path).await?;
        let min_diff = args.min_difference_percentage.unwrap_or(1.0);
        
        let mut opportunities = Vec::new();
        let pools = response.data;

        for i in 0..pools.len() {
            for j in i + 1..pools.len() {
                let pool1 = &pools[i].attributes;
                let pool2 = &pools[j].attributes;

                if let (Some(price1_str), Some(price2_str)) = (
                    &pool1.base_token_price_usd,
                    &pool2.base_token_price_usd,
                ) {
                    let price1: f64 = price1_str.parse().map_err(|_| {
                        GeckoError::InvalidPoolData("Invalid price format".into())
                    })?;
                    let price2: f64 = price2_str.parse().map_err(|_| {
                        GeckoError::InvalidPoolData("Invalid price format".into())
                    })?;
                    
                    let diff_percentage = ((price1 - price2).abs() / price1) * 100.0;
                    
                    if diff_percentage > min_diff {
                        // Calculate potential profit percentage (accounting for typical DEX fees)
                        let potential_profit = diff_percentage - 0.6; // Assuming 0.3% fee per trade
                        
                        opportunities.push(ArbitrageOpportunity {
                            pool1_name: pool1.name.clone(),
                            pool1_address: pool1.address.clone(),
                            pool2_name: pool2.name.clone(),
                            pool2_address: pool2.address.clone(),
                            pool1_price: price1,
                            pool2_price: price2,
                            difference_percentage: diff_percentage,
                            potential_profit_percentage: potential_profit,
                        });
                    }
                }
            }
        }

        Ok(opportunities)
    }
}