use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Account, Asset};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> AccountCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }

    pub fn sponsor(&mut self, sponsor: &str) -> &mut Self {
        self.query_params
            .insert(String::from("sponsor"), String::from(sponsor));

        self
    }

    pub fn signer(&mut self, signer: &str) -> &mut Self {
        self.query_params
            .insert(String::from("signer"), String::from(signer));

        self
    }

    pub fn liquidity_pool(&mut self, liquidity_id: &str) -> &mut Self {
        self.query_params
            .insert(String::from("liquidity_pool"), String::from(liquidity_id));

        self
    }

    pub fn asset(&mut self, asset: &Asset) -> &mut Self {
        self.query_params
            .insert(String::from("asset"), asset.to_string());

        self
    }
}

impl<'a> CallBuilder<Account> for AccountCallBuilder<'a> {
    fn call(&self) -> Result<Record<Account>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/accounts",
        );

        api_call::<Record<Account>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }

    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.query_params
            .insert(String::from("cursor"), String::from(cursor));

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params
            .insert(String::from("order"), dir.to_string());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params
            .insert(String::from("limit"), limit.to_string());

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
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut acb = AccountCallBuilder::new(&s);

        let account_records = acb
            .liquidity_pool("a468d41d8e9b8f3c7209651608b74b7db7ac9952dcae0cdf24871d1d9c7b0088")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(account_records._embedded.records.len(), 3);
    }
}
