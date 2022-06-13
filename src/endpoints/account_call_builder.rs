use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Account, Asset};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> AccountCallBuilder<'a> {
    pub fn sponsor(&mut self, sponsor: &'a str) -> &mut Self {
        self.query_params.insert("sponsor", sponsor);

        self
    }

    pub fn signer(&mut self, signer: &'a str) -> &mut Self {
        self.query_params.insert("signer", signer);

        self
    }

    pub fn liquidity_pool(&mut self, liquidity_id: &'a str) -> &mut Self {
        self.query_params.insert("liquidity_pool", liquidity_id);

        self
    }

    pub fn asset(&mut self, asset: &'a Asset) -> &mut Self {
        self.query_params.insert("asset", &asset.as_str());

        self
    }
}

impl<'a> CallBuilder<'a, Account> for AccountCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        AccountCallBuilder {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
    }

    fn call(&self) -> Result<Record<Account>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades?",
        );

        api_call::<Record<Account>>(url, crate::types::HttpMethod::GET, self.query_params)
    }

    fn cursor(&mut self, cursor: &'a str) -> &mut Self {
        self.query_params.insert("cursor", cursor);

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params.insert("order", dir.as_str());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params.insert("order", &limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_account_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut acb = AccountCallBuilder::new(&s);

        let account_records = acb
            .liquidity_pool("a468d41d8e9b8f3c7209651608b74b7db7ac9952dcae0cdf24871d1d9c7b0088")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(account_records._embedded.records.len(), 3);
    }
}
