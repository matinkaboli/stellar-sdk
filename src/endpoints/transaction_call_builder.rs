use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Transaction;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct TransactionCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> TransactionCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
    }

    pub fn include_failed(&mut self, i: bool) -> &mut Self {
        self.query_params.insert("include_failed", &i.to_string());

        self
    }
}

impl<'a> CallBuilder<'a, Transaction> for TransactionCallBuilder<'a> {
    fn cursor(&mut self, cursor: &'a str) -> &mut Self {
        self.query_params.insert("cursor", cursor);

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params.insert("order", dir.as_str());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params.insert("limit", &limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Transaction>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/transactions",
        );

        api_call::<Record<Transaction>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_transaction_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut tcb = TransactionCallBuilder::new(&s);

        let tx_records = tcb
            .for_endpoint(Endpoint::Accounts(String::from(
                "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
            )))
            .limit(200)
            .call()
            .unwrap();

        assert_eq!(tx_records._embedded.records.len(), 200);
    }
}
