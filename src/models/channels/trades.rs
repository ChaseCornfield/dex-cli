use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Trades{
    pub channel: String,
    pub data: Vec<Data>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Data
{
    pub coin:String, 
    pub hash: String, // hash of trasaction (hype)
    pub px: String, // price
    pub side: String, // A || B , for Ask (Seller) or Bid (buyer)
    pub sz: String, // Size
    pub tid: u64, // traderID
    pub time: u64,
    pub users: [String; 2],
}

