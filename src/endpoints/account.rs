use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::endpoints::Links;

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
pub struct Balance<'a> {
    pub balance: &'a str,
    pub asset_type: &'a str,
    pub limit: Option<&'a str>,
    pub buying_liabilities: Option<&'a str>,
    pub selling_liabilities: Option<&'a str>,
    pub last_modified_ledger: Option<u64>,
    pub liquidity_pool_id: Option<&'a str>,
    pub is_authorized: Option<bool>,
    pub is_authorized_to_maintain_liabilities: Option<bool>,
    pub asset_code: Option<&'a str>,
    pub asset_issuer: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signers<'a> {
    pub weight: u32,
    pub key: &'a str,
    pub r#type: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account<'a> {
    pub _links: Links,
    pub id: &'a str,
    pub account_id: &'a str,
    pub sequence: &'a str,
    pub subentry_count: u32,
    pub inflation_destination: Option<&'a str>,
    pub home_domain: Option<&'a str>,
    pub last_modified_ledger: u64,
    pub last_modified_time: &'a str,
    pub thresholds: Thresholds,
    pub flags: Flags,
    pub balances: Vec<Balance<'a>>,
    pub signers: Vec<Signers<'a>>,
    pub data: HashMap<&'a str, &'a str>,
    pub num_sponsoring: i32,
    pub num_sponsored: i32,
    pub paging_token: &'a str,
}

impl<'a> Account<'a> {
    // pub fn increment_sequence_number(mut self) {
    //     let parsed: u64 = self.sequence.parse().unwrap();
    //     let parsed = parsed + 1;
    //     let parsed = parsed.to_string();
    //     self.sequence = parsed;
    // }

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

    use crate::endpoints::*;

    #[test]
    fn sequence_number_adds() {
        let mut my_account = Account {
                _links: Links {
                    itself: TemplateLink {
                        href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
                        templated: None,
                    },
                    transactions: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/transactions{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    operations: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/operations{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    payments: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/payments{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    effects: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/effects{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    offers: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/offers{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                    trades: Some(
                        TemplateLink {
                            href: "https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/trades{?cursor,limit,order}",
                            templated: Some(
                                true,
                            ),
                        },
                    ),
                },
                id: "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
                account_id: "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
                sequence: "129664371176506169",
                subentry_count: 26,
                inflation_destination: Some(
                    "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
                ),
                home_domain: None,
                last_modified_ledger: 40702343,
                last_modified_time: "2022-05-01T14:18:19Z",
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
                        balance: "370.3906091",
                        asset_type: "credit_alphanum4",
                        limit: Some(
                            "300000.0000000",
                        ),
                        buying_liabilities: Some(
                            "0.0000000",
                        ),
                        selling_liabilities: Some(
                            "0.0000000",
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
                            "AFR",
                        ),
                        asset_issuer: Some(
                            "GBX6YI45VU7WNAAKA3RBFDR3I3UKNFHTJPQ5F6KOOKSGYIAM4TRQN54W",
                        ),
                    },
                ],
                signers: vec![
                    Signers {
                        weight: 2,
                        key: "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
                        r#type: "ed25519_public_key",
                    },
                ],
                data: HashMap::new(),
                num_sponsoring: 0,
                num_sponsored: 0,
                paging_token: "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",         
        };

        println!("{:#?}", my_account);

        let a = my_account.account_id;
        let b = my_account.account_id;

        // let old_sequence = my_account.sequence;

        // my_account.increment_sequence_number();

        // let new_sequence = &mut my_account.sequence;

        // assert_eq!(&old_sequence, new_sequence);
    }
}
