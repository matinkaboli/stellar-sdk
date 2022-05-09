use serde_json;

use crate::endpoints::{
    Account, AccountCallBuilder, AssetCallBuilder, Transaction, TransactionCallBuilder,
};
use crate::utils::{req, Endpoint};

#[derive(Debug)]
pub struct Server(pub String);

impl Server {
    pub fn new(network_id: String) -> Self {
        Server(network_id)
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

    pub fn assets(&self) -> AssetCallBuilder {
        AssetCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            asset_code: None,
            asset_issuer: None,
            endpoint: Endpoint::None,
        }
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

    pub fn load_transaction(&self, hash: &str) -> Result<Transaction, &str> {
        let url = format!("{}/transactions/{}", self.0, hash);
        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: Transaction = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
    }

    pub fn transactions(&self) -> TransactionCallBuilder {
        TransactionCallBuilder {
            server: self,
            cursor: None,
            order: None,
            limit: None,
            include_failed: false,
            endpoint: Endpoint::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_account() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_account("GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM")
            .unwrap();

        assert_eq!(tx.id, tx.account_id);
    }

    #[test]
    fn test_load_transaction() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let tx = s
            .load_transaction("3389e9f0f1a65f19736cacf544c2e825313e8447f569233bb8db39aa607c8889")
            .unwrap();

        assert_eq!(tx.id, tx.hash);
    }
}
