use std::collections::VecDeque;
use crate::models::channels::trades;
use super::constants::MEMORY_TRADE_HISTORY_SIZE;

pub struct MarketState
{
    pub marketPrice: Option<f64>,
    pub assetName: String,
    pub last_trades: VecDeque<trades::Data>,  // last 50 trades

}


impl MarketState {
    pub fn new(asset_name:String) -> Self{
        Self {
                marketPrice: None,
                assetName: asset_name,
                last_trades: VecDeque::new() 
            }
    }

    pub fn get_price(&self) -> f64
    {
        self.marketPrice.unwrap_or(0.0)
    }
    
    pub fn add_trade(&mut self, trade_data: trades::Data)
    {
        //Extract and update price first (while we still have access)
        if let Ok(price) = trade_data.px.parse::<f64>() {
            self.marketPrice = Some(price);
        }
        
        // Then move trade_data into history
        self.last_trades.push_back(trade_data);
        
        //Keep only last 50
        if self.last_trades.len() > MEMORY_TRADE_HISTORY_SIZE {
            self.last_trades.pop_front();
        }
    }


    
}