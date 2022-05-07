use serde::{Deserialize, Serialize};

use crate::endpoints::TemplateLink;

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: Option<TemplateLink>,
    pub account: Option<TemplateLink>,
    pub ledger: Option<TemplateLink>,
    pub operations: Option<TemplateLink>,
    pub effects: Option<TemplateLink>,
    pub precedes: Option<TemplateLink>,
    pub succeeds: Option<TemplateLink>,
    pub transaction: Option<TemplateLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub memo: String,
    pub memo_bytes: String,
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
