use super::subscription::{create_subscription_message};
use tokio_tungstenite::{connect_async, tungstenite};
use futures_util::StreamExt;
use futures_util::SinkExt;
use tungstenite::Message;



const WEBHOOK_ADDRESS: &str = "wss://api.hyperliquid.xyz/ws";
pub async fn websocket()-> Result<(), Box<dyn std::error::Error>>
{
    let (ws_stream, _response) = connect_async(WEBHOOK_ADDRESS).await?;

    let (mut write, mut read) = ws_stream.split();

    // sending message
    let json = serde_json::to_string(&create_subscription_message("BTC".to_string()))?;
    write.send(Message::Text(json)).await?;


    read.next().await;
    Ok(())
}
