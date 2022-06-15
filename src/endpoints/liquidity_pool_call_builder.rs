use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, LiquidityPool};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct LiquidityPoolCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> LiquidityPoolCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
    }

    pub fn for_assets(&mut self, assets: Vec<Asset<'a>>) -> &mut Self {
        self.query_params.insert(
            "reserves",
            &assets
                .into_iter()
                .map(|asset| asset.as_str())
                .collect::<Vec<String>>()
                .join(","),
        );

        self
    }
}

impl<'a> CallBuilder<'a, LiquidityPool> for LiquidityPoolCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<LiquidityPool>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/liquidity_pools",
        );

        api_call::<Record<LiquidityPool>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_liquidity_pools() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut lpcb = LiquidityPoolCallBuilder::new(&s);

        let records = lpcb.limit(200).call().unwrap();

        assert_eq!(records._embedded.records.len(), 200);
    }

    #[test]
    fn test_liquidity_pools_for_assets() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let y_xlm_str = "yXLM:GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55";

        let y_xlm = Asset::new(
            "yXLM",
            "GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55",
        );
        let y_usdc = Asset::new(
            "yUSDC",
            "GDGTVWSM4MGS4T7Z6W4RPWOCHE2I6RDFCIFZGS3DOA63LWQTRNZNTTFF",
        );

        let mut lpcb = LiquidityPoolCallBuilder::new(&s);

        let records = lpcb.for_assets(vec![y_xlm, y_usdc]).call().unwrap();

        assert_eq!(records._embedded.records[0].reserves[0].asset, y_xlm_str);
    }
}
