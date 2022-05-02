use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use ureq;

#[derive(Debug)]
pub struct Asset<'a>(&'a str, &'a str, bool);

impl<'a> Asset<'a> {
    pub fn new(code: &'a str, issuer: &'a str) -> Self {
        Asset(code, issuer, false)
    }

    pub fn as_str(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }

    pub fn native() -> Self {
        Asset("XLM", "", true)
    }
}

impl<'a> Eq for Asset<'a> {}
impl<'a> PartialEq for Asset<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateLink {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: TemplateLink,
    pub next: Option<TemplateLink>,
    pub prev: Option<TemplateLink>,
    pub transactions: Option<TemplateLink>,
    pub operations: Option<TemplateLink>,
    pub payments: Option<TemplateLink>,
    pub effects: Option<TemplateLink>,
    pub offers: Option<TemplateLink>,
    pub trades: Option<TemplateLink>,
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Embedded {
    records: Vec<Account>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountRecord {
    pub _links: Links,
    pub _embedded: Embedded,
}

impl Account {
    pub fn increment_sequence_number(&mut self) {
        let n: u64 = self.sequence.parse().unwrap();

        self.sequence = (n + 1).to_string();
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Presale {
    address: String,
    usdt: String,
    rbt: String,
}

fn req(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url).call()?.into_string()?;

    Ok(body)
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
        }
    }
}

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    server: &'a Server,
    cursor: Option<String>,
    sponsor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
    signer: Option<String>,
    liquidity_pool: Option<String>,
    asset: Option<&'a Asset<'a>>,
}

impl<'a> AccountCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        AccountCallBuilder {
            server: s,
            cursor: None,
            order: None,
            sponsor: None,
            limit: None,
            signer: None,
            liquidity_pool: None,
            asset: None,
        }
    }

    pub fn cursor(&mut self, c: &str) -> &mut Self {
        self.cursor = Some(c.to_owned());

        self
    }

    pub fn sponsor(&mut self, s: &str) -> &mut Self {
        self.sponsor = Some(s.to_owned());

        self
    }

    pub fn order(&mut self, o: Order) -> &mut Self {
        self.order = Some(o);

        self
    }

    pub fn limit(&mut self, l: u32) -> &mut Self {
        self.limit = Some(l);

        self
    }

    pub fn signer(&mut self, s: &str) -> &mut Self {
        self.signer = Some(s.to_owned());

        self
    }

    pub fn liquidity_pool(&mut self, l: &str) -> &mut Self {
        self.liquidity_pool = Some(l.to_owned());

        self
    }

    pub fn asset(&mut self, a: &'a Asset) -> &mut Self {
        self.asset = Some(a);

        self
    }

    pub fn call(&self) -> Result<AccountRecord, &str> {
        let mut url = String::from(&self.server.0);

        url.push_str("/accounts?");

        if let Some(x) = &self.cursor {
            let s = format!("&cursor={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.sponsor {
            let s = format!("&sponsor={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.order {
            let s = format!("&order={}", x.as_str());
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.limit {
            let s = format!("&limit={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.signer {
            let s = format!("&signer={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.liquidity_pool {
            let s = format!("&liquidity_pool={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.asset {
            let s = format!("&asset={}", x.as_str());
            url.push_str(&s[..]);
        }

        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: AccountRecord = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
    }
}

#[derive(Debug)]
pub struct Server(String);

impl Server {
    pub fn new(network_id: String) -> Self {
        Server(network_id)
    }

    pub fn load_account(&self, account_id: &str) -> Result<Account, &str> {
        let url = format!("{}/accounts/{}", self.0, account_id);
        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: Account = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
    }

    pub fn accounts(&self) -> AccountCallBuilder {
        AccountCallBuilder {
            server: self,
            cursor: None,
            order: None,
            sponsor: None,
            limit: None,
            signer: None,
            liquidity_pool: None,
            asset: None,
        }
    }
}

// let content = std::fs::read_to_string("./data/presale.json").unwrap();

// let addresses: Vec<Presale> = serde_json::from_str(&content).unwrap();

// for address in addresses {
//     std::thread::spawn(move || {
//         check_token(&address.address);
//     })
//     .join()
//     .unwrap();
// }

// #[tokio::main]
// async fn check_token(address: &str) -> Result<i16, Box<dyn std::error::Error>> {
//     let rbt = (
//         "RBT",
//         "GCMSCRWZ3QBOI6AF75B5ZWDBXOSMIRW4FSBZH5OI65Y4H4GVH7LPSOYS",
//     );

//     let resp = reqwest::get(format!(
//         "{}{}",
//         "https://horizon.stellar.org/accounts/", address
//     ))
//     .await?
//     .text()
//     .await?;

//     let p: Account = serde_json::from_str(&resp).unwrap();

//     for i in p.balances {
//         if let Some(x) = i.asset_code {
//             if x == rbt.0.to_owned() && i.asset_issuer == Some(rbt.1.to_owned()) {
//                 return Ok(0);
//             }
//         }
//     }

//     println!("Address {} does NOT have trustline", address);
//     Ok(1)
// }
