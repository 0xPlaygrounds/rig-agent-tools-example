use serde::{Deserialize, Serialize};
use serde_json::json;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use crate::{
    client::{GeckoClient, GeckoResponse, TokenPrice},
    error::GeckoError,
};

#[derive(Deserialize, Serialize)]
pub struct PriceArgs {
    network: String,
    addresses: Vec<String>,
}

pub struct PriceTool {
    client: GeckoClient,
}

impl PriceTool {
    pub fn new() -> Self {
        Self {
            client: GeckoClient::new(),
        }
    }
}

impl Default for PriceTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for PriceTool {
    const NAME: &'static str = "check_prices";
    type Error = GeckoError;
    type Args = PriceArgs;
    type Output = std::collections::HashMap<String, String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get current USD prices of multiple tokens on a network".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "network": {
                        "type": "string",
                        "description": "Network ID (e.g., 'eth', 'bsc')"
                    },
                    "addresses": {
                        "type": "array",
                        "items": {
                            "type": "string"
                        },
                        "description": "Array of token addresses to check prices for (max 30)"
                    }
                },
                "required": ["network", "addresses"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if args.addresses.len() > 30 {
            return Err(GeckoError::ApiError("Maximum 30 addresses allowed".into()));
        }

        let addresses = args.addresses.join(",");
        let path = format!("/simple/networks/{}/token_price/{}", args.network, addresses);
        
        let response: GeckoResponse<TokenPrice> = self.client.get(&path).await?;
        Ok(response.data.attributes.token_prices)
    }
}