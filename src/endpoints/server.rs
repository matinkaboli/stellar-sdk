use serde_json;

use crate::endpoints::Account;
use crate::endpoints::AccountCallBuilder;
use crate::utils::req;

#[derive(Debug)]
pub struct Server(pub String);

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
