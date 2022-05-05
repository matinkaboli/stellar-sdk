use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Thresholds {
    pub low_threshold: u8,
    pub med_threshold: u8,
    pub high_threshold: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub auth_required: bool,
    pub auth_revocable: bool,
    pub auth_immutable: bool,
    pub auth_clawback_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub asset_type: String,
    pub limit: Option<String>,
    pub buying_liabilities: Option<String>,
    pub selling_liabilities: Option<String>,
    pub last_modified_ledger: Option<u64>,
    pub liquidity_pool_id: Option<String>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    pub balance: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signers {
    pub weight: u32,
    pub key: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub _links: Links,
    pub id: String,
    pub account_id: String,
    pub sequence: String,
    pub subentry_count: u32,
    pub inflation_destination: Option<String>,
    pub home_domain: Option<String>,
    pub last_modified_ledger: u64,
    pub last_modified_time: String,
    pub thresholds: Thresholds,
    pub flags: Flags,
    pub balances: Vec<Balance>,
    pub signers: Vec<Signers>,
    pub data: HashMap<String, String>,
    pub num_sponsoring: i32,
    pub num_sponsored: i32,
    pub paging_token: String,
}

impl Account {
    pub fn increment_sequence_number(&mut self) {
        let n: u64 = self.sequence.parse().unwrap();

        self.sequence = (n + 1).to_string();
    }

    pub fn transactions(&self) {}

    pub fn operations(&self) {}

    pub fn payments(&self) {}

    pub fn effects(&self) {}

    pub fn offers(&self) {}

    pub fn trades(&self) {}
}
