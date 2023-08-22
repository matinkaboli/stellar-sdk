use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, LiquidityPool};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct LiquidityPoolCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> LiquidityPoolCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }

    pub fn for_assets(&mut self, assets: Vec<Asset>) -> &mut Self {
        self.query_params.insert(
            String::from("reserves"),
            assets
                .into_iter()
                .map(|asset| asset.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );

        self
    }
}

impl<'a> CallBuilder<LiquidityPool> for LiquidityPoolCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<LiquidityPool>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/liquidity_pools",
        );

        api_call::<Record<LiquidityPool>>(
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
    fn test_limit_liquidity_pools() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut lpcb = LiquidityPoolCallBuilder::new(&s);

        let records = lpcb.limit(200).call().unwrap();

        assert_eq!(records._embedded.records.len(), 200);
    }

    #[test]
    fn test_liquidity_pools_for_assets() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let y_xlm_str = "yXLM:GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55";

        let y_xlm = Asset::new(
            String::from("yXLM"),
            String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
        )
        .unwrap();
        let y_usdc = Asset::new(
            String::from("yUSDC"),
            String::from("GDGTVWSM4MGS4T7Z6W4RPWOCHE2I6RDFCIFZGS3DOA63LWQTRNZNTTFF"),
        )
        .unwrap();

        let mut lpcb = LiquidityPoolCallBuilder::new(&s);

        let records = lpcb.for_assets(vec![y_xlm, y_usdc]).call().unwrap();

        assert_eq!(records._embedded.records[0].reserves[0].asset, y_xlm_str);
    }
}
