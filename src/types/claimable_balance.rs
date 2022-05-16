use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{Claimant, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimableBalanceLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub transactions: ResponseLink,
    pub operations: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimableBalanceFlags {
    pub clawback_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimableBalance {
    pub _links: ClaimableBalanceLinks,
    pub id: String,
    pub asset: String,
    pub amount: String,
    pub sponsor: Option<String>,
    pub last_modified_ledger: u64,
    pub last_modified_time: String,
    pub claimants: Vec<Claimant>,
    pub paging_token: String,
    pub flags: ClaimableBalanceFlags,
}
