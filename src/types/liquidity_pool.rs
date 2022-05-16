use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{Reserve, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidityPoolLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub transactions: ResponseLink,
    pub operations: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidityPool {
    pub _links: LiquidityPoolLinks,
    pub id: String,
    pub paging_token: String,
    pub fee_bp: u64,
    pub r#type: String,
    pub total_trustlines: String,
    pub total_shares: String,
    pub reserves: Vec<Reserve>,
    pub last_modified_ledger: u64,
    pub last_modified_time: String,
}
