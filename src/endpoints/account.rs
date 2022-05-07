use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::endpoints::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: TemplateLink,
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
    pub balance: String,
    pub asset_type: String,
    pub limit: Option<String>,
    pub buying_liabilities: Option<String>,
    pub selling_liabilities: Option<String>,
    pub last_modified_ledger: Option<u64>,
    pub liquidity_pool_id: Option<String>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
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
    pub _links: AccountLinks,
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
        let mut new_sequence: u64 = self.sequence.parse().unwrap();
        new_sequence += 1;

        let new_sequence = new_sequence.to_string();

        self.sequence = new_sequence;
    }

    pub fn transactions(&self) {}

    pub fn operations(&self) {}

    pub fn payments(&self) {}

    pub fn effects(&self) {}

    pub fn offers(&self) {}

    pub fn trades(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_number_adds() {
        let mut my_account = Account {
                _links: AccountLinks {
                    itself: TemplateLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                        templated: None,
                    },
                    transactions: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/transactions{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    operations: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/operations{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    payments: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/payments{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    effects: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/effects{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    offers: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/offers{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    trades: Some(
                        TemplateLink {
                            href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/trades{?cursor,limit,order}"),
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                },
                id: String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                account_id: String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                sequence: String::from("129664371176506169"),
                subentry_count: 26,
                inflation_destination: Some(
                    String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                ),
                home_domain: None,
                last_modified_ledger: 40702343,
                last_modified_time: String::from("2022-05-01T14:18:19Z"),
                thresholds: Thresholds {
                    low_threshold: 1,
                    med_threshold: 1,
                    high_threshold: 1,
                },
                flags: Flags {
                    auth_required: true,
                    auth_revocable: true,
                    auth_immutable: true,
                    auth_clawback_enabled: true,
                },
                balances: vec![
                    Balance {
                        balance: String::from("370.3906091"),
                        asset_type: String::from("credit_alphanum4"),
                        limit: Some(
                            String::from("300000.0000000"),
                        ),
                        buying_liabilities: Some(
                            String::from("0.0000000"),
                        ),
                        selling_liabilities: Some(
                            String::from("0.0000000"),
                        ),
                        last_modified_ledger: Some(
                            40304840,
                        ),
                        liquidity_pool_id: None,
                        is_authorized: Some(
                            true,
                        ),
                        is_authorized_to_maintain_liabilities: Some(
                            true,
                        ),
                        asset_code: Some(
                            String::from("AFR"),
                        ),
                        asset_issuer: Some(
                            String::from("GBX6YI45VU7WNAAKA3RBFDR3I3UKNFHTJPQ5F6KOOKSGYIAM4TRQN54W"),
                        ),
                    },
                ],
                signers: vec![
                    Signers {
                        weight: 2,
                        key: String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                        r#type: String::from("ed25519_public_key"),
                    },
                ],
                data: HashMap::new(),
                num_sponsoring: 0,
                num_sponsored: 0,
                paging_token: String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),         
        };

        my_account.increment_sequence_number();

        let seq = String::from("129664371176506169");
        let mut seq: u64 = seq.parse().unwrap();
        seq += 1;
        let seq = seq.to_string();

        assert_eq!(seq, my_account.sequence);
    }
}
