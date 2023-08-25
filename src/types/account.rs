use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::endpoints::horizon::{AccountSigner, AccountThresholds, Balance, Flags, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub transactions: ResponseLink,
    pub operations: ResponseLink,
    pub payments: ResponseLink,
    pub effects: ResponseLink,
    pub offers: ResponseLink,
    pub trades: ResponseLink,
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
    pub thresholds: AccountThresholds,
    pub flags: Flags,
    pub balances: Vec<Balance>,
    pub signers: Vec<AccountSigner>,
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

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn sequence_number(&self) -> &str {
        &self.sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_number_adds() {
        let mut my_account = Account {
                _links: AccountLinks {
                    itself: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                        templated: None,
                    },
                    transactions: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/transactions{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
                    operations: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/operations{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
                    payments: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/payments{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
                    effects: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/effects{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
                    offers: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/offers{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
                    trades: ResponseLink {
                        href: String::from("https://horizon.stellar.org/accounts/GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM/trades{?cursor,limit,order}"),
                        templated: Some(
                            true,
                            ),
                    },
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
                thresholds: AccountThresholds {
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
                        limit: Some(
                            String::from("300000.0000000"),
                        ),
                        asset_type: String::from("credit_alphanum4"),
                        asset_code: Some(
                            String::from("AFR"),
                        ),
                        asset_issuer: Some(
                            String::from("GBX6YI45VU7WNAAKA3RBFDR3I3UKNFHTJPQ5F6KOOKSGYIAM4TRQN54W"),
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
                        is_clawback_enabled: Some(false),
                    },
                ],
                signers: vec![
                    AccountSigner {
                        weight: 2,
                        key: String::from("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM"),
                        r#type: String::from("ed25519_public_key"),
                        sponsor: None,
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
