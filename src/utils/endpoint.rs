#[derive(Debug)]
pub enum Endpoint {
    None,
    Ledgers(String),
    Accounts(String),
    LiquidityPools(String),
    ClaimableBalances(String),
}

impl Endpoint {
    pub fn as_str(&self) -> String {
        match self {
            Endpoint::None => format!(""),
            Endpoint::Ledgers(s) => format!("{}{}", "/ledgers/", s),
            Endpoint::Accounts(s) => format!("{}{}", "/accounts/", s),
            Endpoint::LiquidityPools(s) => format!("{}{}", "/liquidity_pools/", s),
            Endpoint::ClaimableBalances(s) => format!("{}{}", "/claimable_balances/", s),
        }
    }
}
