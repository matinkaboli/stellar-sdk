use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::ResponseLink;

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: Option<ResponseLink>,
    pub account: Option<ResponseLink>,
    pub ledger: Option<ResponseLink>,
    pub operations: Option<ResponseLink>,
    pub effects: Option<ResponseLink>,
    pub precedes: Option<ResponseLink>,
    pub succeeds: Option<ResponseLink>,
    pub transaction: Option<ResponseLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub memo: Option<String>,
    pub memo_bytes: Option<String>,
    pub _links: TransactionLinks,
    pub id: String,
    pub paging_token: String,
    pub successful: bool,
    pub hash: String,
    pub ledger: u64,
    pub created_at: String,
    pub source_account: String,
    pub source_account_sequence: String,
    pub fee_account: String,
    pub fee_charged: String,
    pub max_fee: String,
    pub operation_count: u64,
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub result_meta_xdr: String,
    pub fee_meta_xdr: String,
    pub memo_type: String,
    pub signatures: Vec<String>,
    pub valid_after: Option<String>,
    pub valid_before: Option<String>,
}
