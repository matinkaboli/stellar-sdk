use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseLink {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub next: Option<ResponseLink>,
    pub prev: Option<ResponseLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Embedded<T> {
    pub records: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record<T> {
    pub _links: Option<RecordLinks>,
    pub _embedded: Embedded<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Predicate {
    pub and: Option<Vec<Predicate>>,
    pub or: Option<Vec<Predicate>>,
    pub not: Option<Box<Predicate>>,
    pub abs_before: Option<String>,
    pub rel_before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claimant {
    pub destination: String,
    pub predicate: Predicate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub balance: String,
    pub limit: Option<String>,
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub buying_liabilities: Option<String>,
    pub selling_liabilities: Option<String>,
    pub last_modified_ledger: Option<u64>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    pub is_clawback_enabled: Option<bool>,
    pub liquidity_pool_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetAccounts {
    pub authorized: u32,
    pub authorized_to_maintain_liabilities: u32,
    pub unauthorized: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetBalances {
    pub authorized: String,
    pub authorized_to_maintain_liabilities: String,
    pub unauthorized: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceR {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceRShortHand<T> {
    pub n: T,
    pub d: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountThresholds {
    pub low_threshold: u8,
    pub med_threshold: u8,
    pub high_threshold: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub auth_required: bool,
    pub auth_revocable: bool,
    pub auth_immutable: bool,
    #[serde(default)] // Backwards compatibility with protocol version 15 for example where this auth_clawback_enabled field not existing
    pub auth_clawback_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountSigner {
    pub weight: u32,
    pub key: String,
    pub r#type: String,
    pub sponsor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reserve {
    pub asset: String,
    pub amount: String,
    pub claimable_balance_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub asset_type: String,
}
