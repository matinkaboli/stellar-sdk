#[derive(Debug)]
pub enum TradeType {
    All,
    LiquidityPools,
    Orderbook,
}

impl TradeType {
    pub fn as_str(&self) -> String {
        match self {
            TradeType::All => String::from("all"),
            TradeType::LiquidityPools => String::from("liquidity_pool"),
            TradeType::Orderbook => String::from("orderbook"),
        }
    }
}
