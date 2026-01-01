use crate::market;

use super::subscription::create_subscription_message;
use super::message_handler::{parse_message, IncomingMessage};
use super::constants::{WEBSOCKET_ADDRESS, CHANNEL_BBO, CHANNEL_TRADES};
use tokio_tungstenite::{connect_async, tungstenite};
use futures_util::StreamExt;
use futures_util::SinkExt;
use futures_util::Sink;
use tungstenite::Message;
use crate::market::marketstate::MarketState;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{self, Write};

/// I created this function to be reused and for better modularity
/// #Returns the formatted message if corrected called, or None if no data present or websocket error or a 
/// Non text message print if the text received is binary
fn format_sub(msg: Option<Result<Message, tungstenite::Error>>,
                market_state: &mut MarketState,)
                -> Result<(), Box<dyn std::error::Error>>
        {
    match msg {
        None => {
            println!("Stream ended (None)");
        }
            // if some OK (accepted) message is passed we use serde json with to string pretty to format it 
            // nicely for reading
        Some(Ok(Message::Text(text))) => 
        {
            // after checking if the message is acceptable we will do another match to find out its type
            // and base on that perform the struct construction!
            match parse_message(&text) 
            {
                Ok(IncomingMessage::Bbo { data }) => 
                {
                   // println!("BBO: {:?}", data); 
                   print!("")
                }
                Ok(IncomingMessage::Trades { data }) => 
                {
                    // Add each trade to market state
                    for trade in data {
                        market_state.add_trade(trade);
                    }
                    // Maybe print current price for now
                    io::stdout()
                    .execute(cursor::MoveToColumn(0))?
                    .execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                    print!("Updated price: ${:.2}", market_state.get_price());
                    io::stdout().flush()?;
                }
                Ok(IncomingMessage::SubscriptionResponse { .. }) => {
                    // Subscription confirmed, do nothing
                }
                Err(e) => {
                    // Ignore subscription confirmations
                    if !text.contains("subscriptionResponse") {
                        println!("Could not parse: {} - raw: {}", e, text);
                    }
                }

            }
        }
        // otherwise it returts one of this message based on the most commum error possibilites
        Some(Ok(other)) => {
            println!("Non-text message: {:?}", other);
        }
        Some(Err(e)) => {
            println!("WebSocket error: {:?}", e);
        }
    }

    Ok(())
}

/// THis async function was created so we can easily and with maintainalibility create multiple subscriptions
/// at once, to achieve our goal
async fn subscribe(
    write: &mut (impl Sink<Message, Error = tungstenite::Error> + Unpin),
    subscription_kind: &str,
    asset_name: &str,
) -> Result<(), Box<dyn std::error::Error>>
{
    let subscription_message = serde_json::to_string(&create_subscription_message(
        subscription_kind.to_string(),
        asset_name.to_string(),
    ))?;
    write.send(Message::Text(subscription_message)).await?;
    Ok(())
}

/// here is the main funciton of the file, is where we create the websocket screm and with help of Sink we 
/// start #Streams simuntaniously
pub async fn websocket() -> Result<(), Box<dyn std::error::Error>> 
{
    let (ws_stream, _response) = connect_async(WEBSOCKET_ADDRESS).await?;

    let (mut write, mut read) = ws_stream.split();

    // Create the state tracker here
    let mut market_state = MarketState::new("BTC".to_string());
    
    subscribe(&mut write, CHANNEL_BBO, "BTC").await?; // sub for bids and ask price BTC
    subscribe(&mut write, CHANNEL_TRADES, "BTC").await?; // sub for last traded price BTC

    // using while let, so we keep receving responses while the stream is active, insataed of only once
    // with (if let)
    while let Some(msg) = read.next().await
    { 
        format_sub(Some(msg), &mut market_state)?;
    }

    Ok(())
}
