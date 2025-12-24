use serde::Serialize;
use super::constants::SUBSCRIBE_METHOD;

#[derive(Serialize)]
pub struct SubscriptionMessage {
    pub method: &'static str,
    pub subscription: Subscription,
}

#[derive(Serialize)]
pub struct Subscription {
    #[serde(rename = "type")]  // must be literal, see constants::SUB_TYPE_LABEL
    pub kind: String,
    pub coin: String,
}

/// Function to create the subscription message type to facilitate the process
pub fn create_subscription_message(sub_kind: String, coin_name:String) -> SubscriptionMessage
{
    let sub = 
    Subscription 
    {
        kind: sub_kind,
        coin: coin_name,
    };
    
    let sub_message = 
    SubscriptionMessage
    {
        method: SUBSCRIBE_METHOD,
        subscription:sub,
    };

    return sub_message;
}