use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct TemplateLink {
    href: String,
    templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Links {
    s: TemplateLink,
    transactions: TemplateLink,
    operations: TemplateLink,
    payments: TemplateLink,
    effects: TemplateLink,
    offers: TemplateLink,
    trades: TemplateLink,
    data: TemplateLink,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thresholds {
    low_threshold: u8,
    med_threshold: u8,
    high_threshold: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Flags {
    auth_required: bool,
    auth_revocable: bool,
    auth_immutable: bool,
    auth_clawback_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct NativeAsset {
    asset_type: String,
    selling_liabilities: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreditAlphanum<T> {
    asset_type: T,
    limit: String,
    buying_liabilities: String,
    selling_liabilities: String,
    last_modified_ledger: String,
    is_authorized_to_maintain_liabilities: bool,
    balance: String,
    asset_code: String,
    asset_issuer: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Balances {
    NativeAsset,
    CreditAlphanum,
}

#[derive(Serialize, Deserialize, Debug)]
struct Signers {
    weight: u32,
    key: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    _links: Links,
    id: String,
    account_id: String,
    sequence: String,
    subentry_count: u64,
    inflation_destination: String,
    last_modified_ledger: u64,
    last_modified_time: String,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balances>,
    signers: Vec<Signers>,
    data: HashMap<String, String>,
    num_sponsoring: i32,
    num_sponsored: i32,
    paging_token: String,
}
