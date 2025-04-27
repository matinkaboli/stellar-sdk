use serde::{Deserialize, Serialize};
// https://developers.stellar.org/api/horizon/resources/submit-a-transaction

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTransactionResponse {
    pub memo: Option<String>,
    pub memo_bytes: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub id: String,
    pub paging_token: String,
    pub successful: bool,
    pub hash: String,
    pub ledger: i32,
    pub created_at: String,
    pub source_account: String,
    pub account_muxed: Option<String>,
    pub account_muxed_id: Option<String>,
    pub source_account_sequence: String,
    pub fee_account: String,
    pub fee_account_muxed: Option<String>,
    pub fee_account_muxed_id: Option<String>,
    pub fee_charged: String,
    pub max_fee: String,
    pub operation_count: i32,
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub result_meta_xdr: String,
    pub fee_meta_xdr: Option<String>,
    pub memo_type: String,
    pub signatures: Vec<String>,
    pub valid_after: Option<String>,
    pub valid_before: Option<String>,
    pub preconditions: Option<Preconditions>,
    pub fee_bump_transaction: Option<FeeBumpTransaction>,
    pub inner_transaction: Option<InnerTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub account: Link,
    pub ledger: Link,
    pub operations: Link,
    pub effects: Link,
    pub precedes: Link,
    pub succeeds: Link,
    pub transaction: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preconditions {
    pub timebounds: Option<TimeboundsString>,
    pub ledgerbounds: Option<Ledgerbounds>,
    pub min_account_sequence: Option<String>,
    pub min_account_sequence_age: Option<String>,
    pub min_account_sequence_ledger_gap: Option<i32>,
    pub extra_signers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledgerbounds {
    pub min_ledger: Option<String>,
    pub max_ledger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeboundsString {
    pub min_time: Option<String>,
    pub max_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeBumpTransaction {
    pub hash: Option<String>,
    pub signatures: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerTransaction {
    pub hash: Option<String>,
    pub signatures: Option<Vec<String>>,
    pub max_fee: Option<String>,
}
