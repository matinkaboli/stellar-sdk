use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, LiquidityPool};
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct LiquidityPoolCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub endpoint: Endpoint,
    pub assets: Option<Vec<Asset<'a>>>,
}

impl<'a> LiquidityPoolCallBuilder<'a> {
    pub fn for_assets(&mut self, assets: Vec<Asset<'a>>) -> &mut Self {
        self.assets = Some(assets);

        self
    }
}

impl<'a> CallBuilder<'a, LiquidityPool> for LiquidityPoolCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            assets: None,
        }
    }

    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.cursor = Some(String::from(cursor));

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.order = Some(dir);

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<LiquidityPool>, &str> {
        let mut url = format!(
            "{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/liquidity_pools?",
        );

        if let Some(x) = &self.cursor {
            url.push_str(&format!("&cursor={}", x));
        }

        if let Some(x) = &self.order {
            url.push_str(&format!("&order={}", x.as_str()));
        }

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.assets {
            url.push_str("&reserves=");

            for i in x {
                url.push_str(&format!("{},", i.as_str()))
            }
        }

        let resp = req(&url).unwrap();

        let p: Record<LiquidityPool> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_liquidity_pools() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut lpcb = LiquidityPoolCallBuilder::new();

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

        let mut lpcb = LiquidityPoolCallBuilder::new();

        let records = lpcb.for_assets(vec![y_xlm, y_usdc]).call().unwrap();

        assert_eq!(records._embedded.records[0].reserves[0].asset, y_xlm_str);
    }
}
