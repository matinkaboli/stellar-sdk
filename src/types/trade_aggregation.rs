use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::PriceRShortHand;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeAggregation {
    pub timestamp: String,
    pub trade_count: String,
    pub base_volume: String,
    pub counter_volume: String,
    pub avg: String,
    pub high: String,
    pub high_r: PriceRShortHand<String>,
    pub low: String,
    pub low_r: PriceRShortHand<String>,
    pub open: String,
    pub open_r: PriceRShortHand<String>,
    pub close: String,
    pub close_r: PriceRShortHand<String>,
}
