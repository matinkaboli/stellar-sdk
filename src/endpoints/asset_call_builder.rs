use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::AssetHorizon;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct AssetCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> AssetCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        AssetCallBuilder {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }

    pub fn asset_code(&mut self, code: &str) -> &mut Self {
        self.query_params
            .insert(String::from("code"), String::from(code));

        self
    }

    pub fn asset_issuer(&mut self, issuer: &str) -> &mut Self {
        self.query_params
            .insert(String::from("issuer"), String::from(issuer));

        self
    }
}

impl<'a> CallBuilder<AssetHorizon> for AssetCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<AssetHorizon>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/assets",
        );

        api_call::<Record<AssetHorizon>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut acb = AssetCallBuilder::new(&s);

        let asset_records = acb
            .asset_issuer("GA5BUT4SND34VRUJGFEVLG6LMEYOU5HSSYZLX673I2IJVRLLPATMH4RN")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(asset_records._embedded.records.len(), 3);
    }
}
