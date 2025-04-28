use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{AssetAccounts, AssetBalances, Flags, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetHorizonLinks {
    pub toml: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetHorizon {
    pub _links: AssetHorizonLinks,
    pub asset_type: String,
    pub asset_code: String,
    pub asset_issuer: String,
    pub paging_token: String,
    pub claimable_balances_amount: String,
    pub num_claimable_balances: u32,
    pub liquidity_pools_amount: String,
    pub num_liquidity_pools: u32,
    pub accounts: AssetAccounts,
    pub balances: AssetBalances,
    pub flags: Flags,
    pub num_contracts: u32,
    pub contracts_amount: String,
}
