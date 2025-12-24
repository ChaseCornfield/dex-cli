use serde::Deserialize;
use super::channels::{bbo, trades};

// serde attributes require string literals, can't use constants
// see constants.rs for reference values: CHANNEL_BBO, CHANNEL_TRADES, etc.

#[derive(Deserialize)]
#[serde(tag = "channel")] // serde looks at "channel" field to pick variant
pub enum IncomingMessage 
{
    #[serde(rename = "bbo")]  // constants::CHANNEL_BBO
    Bbo { data: bbo::Data },      

    #[serde(rename = "trades")]  // constants::CHANNEL_TRADES
    Trades { data: trades::Data }, 
}

pub fn parse_message(text: &str) -> Result<IncomingMessage, Box<dyn std::error::Error>>{

    let msg = serde_json::from_str(text)?;
    Ok(msg)
}