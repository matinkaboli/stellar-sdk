use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::PriceRShortHand;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookPriceData {
    pub price: String,
    pub amount: String,
    pub price_r: PriceRShortHand<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookAsset {
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBook {
    pub bids: Vec<OrderBookPriceData>,
    pub asks: Vec<OrderBookPriceData>,
    pub base: OrderBookAsset,
    pub counter: OrderBookAsset,
}
