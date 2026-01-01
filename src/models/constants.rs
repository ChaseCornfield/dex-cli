// Subscription constants
pub const SUBSCRIBE_METHOD: &str = "subscribe";
#[allow(dead_code)]  // reference with serde rename
pub const SUB_TYPE_LABEL: &str = "type";

// Channel names (serde attributes need literals, these are for regular code)
pub const CHANNEL_BBO: &str = "bbo";
pub const CHANNEL_TRADES: &str = "trades";
#[allow(dead_code)]  //future use
pub const CHANNEL_SUB_RESPONSE: &str = "subscriptionResponse";

// Websocket
pub const WEBSOCKET_ADDRESS: &str = "wss://api.hyperliquid.xyz/ws";
