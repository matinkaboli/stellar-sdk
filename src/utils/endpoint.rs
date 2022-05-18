#[derive(Debug)]
pub enum Endpoint {
    None,
    Ledgers(String),
    Accounts(String),
    LiquidityPools(String),
    ClaimableBalances(String),
    Transactions(String),
    Operations(String),
    Other(String, String),
}

impl Endpoint {
    pub fn as_str(&self) -> String {
        match self {
            Endpoint::None => String::new(),
            Endpoint::Ledgers(s) => format!("{}{}", "/ledgers/", s),
            Endpoint::Accounts(s) => format!("{}{}", "/accounts/", s),
            Endpoint::LiquidityPools(s) => format!("{}{}", "/liquidity_pools/", s),
            Endpoint::ClaimableBalances(s) => format!("{}{}", "/claimable_balances/", s),
            Endpoint::Transactions(s) => format!("{}{}", "/transactions", s),
            Endpoint::Operations(s) => format!("{}{}", "/operations", s),
            Endpoint::Other(route, s) => format!("/{}{}", route, s),
        }
    }
}
