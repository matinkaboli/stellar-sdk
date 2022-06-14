use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, ClaimableBalance};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct ClaimableBalanceCallbuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> ClaimableBalanceCallbuilder<'a> {
    pub fn sponsor(&mut self, sponsor: &'a str) -> &mut Self {
        self.query_params.insert("sponsor", sponsor);

        self
    }

    pub fn asset(&mut self, asset: &'a Asset<'a>) -> &mut Self {
        self.query_params.insert("asset", &asset.as_str());

        self
    }

    pub fn claimant(&mut self, claimant: &'a str) -> &mut Self {
        self.query_params.insert("claimant", claimant);

        self
    }
}

impl<'a> CallBuilder<'a, ClaimableBalance> for ClaimableBalanceCallbuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
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

    fn call(&self) -> Result<Record<ClaimableBalance>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades?",
        );

        api_call::<Record<ClaimableBalance>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claimable_balance_sponsor() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let cbcb = ClaimableBalanceCallbuilder::new(&s)
            .sponsor("GDCJIHD3623OCYNH65UUQC3NLG2D6YCNCDPZULRLCLOA76TBQRL6A3TF")
            .limit(1)
            .call()
            .unwrap();

        if let Some(x) = &cbcb._embedded.records[0].sponsor {
            assert_eq!(
                "GDCJIHD3623OCYNH65UUQC3NLG2D6YCNCDPZULRLCLOA76TBQRL6A3TF",
                x
            );
        }
    }

    #[test]
    fn test_claimable_balance_for_asset() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let lsp = Asset::new(
            "LSP",
            "GAB7STHVD5BDH3EEYXPI3OM7PCS4V443PYB5FNT6CFGJVPDLMKDM24WK",
        );

        let cbcb = ClaimableBalanceCallbuilder::new(&s)
            .asset(&lsp)
            .limit(1)
            .call()
            .unwrap();

        assert_eq!(lsp.as_str(), cbcb._embedded.records[0].asset);
    }
}
