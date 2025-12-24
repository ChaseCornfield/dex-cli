use serde::Deserialize;

/// struct for top level BBO
/// this needs to match the json we receive:
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Bbo {
    pub channel: String,
    pub data: Data,
}

/// Struct for mid level (this is the "data" object in the json)
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Data {
    pub coin: String, // coin
    pub time: u64,    // time -unix ms)
    pub bbo: Vec<Order>, // list of 2 entries: best bid and best ask
}

/// low level of the type one entry inside data.bbo
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Order {
    pub n: u32,      // number of orders at this price level
    pub px: String,  // px 
    pub sz: String,  // sz 
}

