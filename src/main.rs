use rig::{
    cli_chatbot::cli_chatbot,
    providers::openai,
};
use crate::tools::{ArbitrageTool, PriceTool};
use std::error::Error;

mod client;
mod error;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Initialize OpenAI client
    let openai_client = openai::Client::from_env();

    // Create agent with both tools
    let crypto_agent = openai_client
        .agent("gpt-4")
        .preamble(
            "You are a crypto market analysis assistant. You can:\n\
            1. Check token prices across networks\n\
            2. Find arbitrage opportunities between different pools\n\
            \n\
            When finding arbitrage opportunities:\n\
            - Consider fees and slippage\n\
            - Only suggest opportunities with significant profit potential\n\
            - Always include both pool addresses for verification\n\
            \n\
            Format monetary values cleanly and provide clear, concise analysis.\n\
            \n\
            Example commands:\n\
            - \"Check WETH price on Ethereum: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2\"\n\
            - \"Find arbitrage for USDC on Ethereum: 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48\"\n\
            - \"Compare WETH and USDC prices on Ethereum\"\n\
            \n\
            Type 'exit' to quit."
        )
        .tool(PriceTool::new())
        .tool(ArbitrageTool::new())
        .build();

    println!("🤖 Crypto Market Analysis Agent");
    println!("-------------------------------");
    println!("Ask me to check prices or find arbitrage opportunities!");
    println!("Type 'exit' to quit\n");

    cli_chatbot(crypto_agent).await?;

    Ok(())
}