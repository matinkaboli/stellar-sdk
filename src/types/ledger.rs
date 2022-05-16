use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::ResponseLink;

#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub transactions: ResponseLink,
    pub operations: ResponseLink,
    pub payments: ResponseLink,
    pub effects: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ledger {
    pub _links: LedgerLinks,
    pub id: String,
    pub paging_token: String,
    pub hash: String,
    pub prev_hash: String,
    pub sequence: u32,
    pub successful_transaction_count: u32,
    pub failed_transaction_count: u32,
    pub operation_count: u32,
    pub tx_set_operation_count: u32,
    pub closed_at: String,
    pub total_coins: String,
    pub fee_pool: String,
    pub base_fee_in_stroops: u32,
    pub base_reserve_in_stroops: u64,
    pub max_tx_set_size: u32,
    pub protocol_version: u32,
    pub header_xdr: String,
}
