use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::AssetHorizon;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct AssetCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> AssetCallBuilder<'a> {
    pub fn asset_code(&mut self, code: &'a str) -> &mut Self {
        self.query_params.insert("code", code);

        self
    }

    pub fn asset_issuer(&mut self, issuer: &'a str) -> &mut Self {
        self.query_params.insert("issuer", issuer);

        self
    }
}

impl<'a> CallBuilder<'a, AssetHorizon> for AssetCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        AssetCallBuilder {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
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

    fn call(&self) -> Result<Record<AssetHorizon>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades?",
        );

        api_call::<Record<AssetHorizon>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut acb = AssetCallBuilder::new(&s);

        let asset_records = acb
            .asset_issuer("GA5BUT4SND34VRUJGFEVLG6LMEYOU5HSSYZLX673I2IJVRLLPATMH4RN")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(asset_records._embedded.records.len(), 3);
    }
}
