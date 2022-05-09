use serde::{Deserialize, Serialize};

use crate::endpoints::Flags;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsAssetHorizon {
    pub authorized: u32,
    pub authorized_to_maintain_liabilities: u32,
    pub unauthorized: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalancesAssetHorizon {
    pub authorized: String,
    pub authorized_to_maintain_liabilities: String,
    pub unauthorized: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetTomlLinks {
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetHorizonLinks {
    pub toml: AssetTomlLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetHorizon {
    pub _links: AssetHorizonLinks,
    pub asset_type: String,
    pub asset_code: String,
    pub asset_issuer: String,
    pub paging_token: String,
    pub num_accounts: u32,
    pub num_claimable_balances: u32,
    pub num_liquidity_pools: u32,
    pub amount: String,
    pub accounts: AccountsAssetHorizon,
    pub claimable_balances_amount: String,
    pub liquidity_pools_amount: String,
    pub balances: BalancesAssetHorizon,
    pub flags: Flags,
}
