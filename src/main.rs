use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct TemplateLink {
    href: String,
    templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Links {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    itself: TemplateLink,
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
struct Balance {
    asset_type: String,
    limit: Option<String>,
    buying_liabilities: Option<String>,
    selling_liabilities: Option<String>,
    last_modified_ledger: Option<u64>,
    liquidity_pool_id: Option<String>,
    is_authorized: Option<bool>,
    is_authorized_to_maintain_liabilities: Option<bool>,
    balance: String,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
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
    subentry_count: u32,
    inflation_destination: Option<String>,
    home_domain: Option<String>,
    last_modified_ledger: u64,
    last_modified_time: String,
    thresholds: Thresholds,
    flags: Flags,
    balances: Vec<Balance>,
    signers: Vec<Signers>,
    data: HashMap<String, String>,
    num_sponsoring: i32,
    num_sponsored: i32,
    paging_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Presale {
    address: String,
    usdt: String,
    rbt: String,
}

#[tokio::main]
async fn main() {
    let content = std::fs::read_to_string("./data/presale.json").unwrap();

    let addresses: Vec<Presale> = serde_json::from_str(&content).unwrap();

    for address in addresses {
        std::thread::spawn(move || {
            check_token(&address.address);
        })
        .join()
        .unwrap();
    }
}

#[tokio::main]
async fn check_token(address: &str) -> Result<i16, Box<dyn std::error::Error>> {
    let rbt = (
        "RBT",
        "GCMSCRWZ3QBOI6AF75B5ZWDBXOSMIRW4FSBZH5OI65Y4H4GVH7LPSOYS",
    );

    let resp = reqwest::get(format!(
        "{}{}",
        "https://horizon.stellar.org/accounts/", address
    ))
    .await?
    .text()
    .await?;

    let p: AccountResponse = serde_json::from_str(&resp).unwrap();

    for i in p.balances {
        if let Some(x) = i.asset_code {
            if x == rbt.0.to_owned() && i.asset_issuer == Some(rbt.1.to_owned()) {
                return Ok(0);
            }
        }
    }

    println!("Address {} does NOT have trustline", address);
    Ok(1)
}
