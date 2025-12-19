use serde::Serialize;

#[derive(Serialize)]
pub struct SubscriptionMessage {
    pub method: &'static str,
    pub subscription: Subscription,
}

#[derive(Serialize)]
pub struct Subscription {
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub coin: String,
}

pub fn create_subscription_message(coin_name:String) -> SubscriptionMessage
{
    let sub = 
    Subscription 
    {
        kind: "bbo",
        coin: coin_name.to_string(),
    };
    
    let sub_message = 
    SubscriptionMessage
    {
        method: "subscribe",
        subscription:sub,
    };

    return sub_message;
}