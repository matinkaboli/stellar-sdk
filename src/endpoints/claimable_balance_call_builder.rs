use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, ClaimableBalance};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct ClaimableBalanceCallbuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> ClaimableBalanceCallbuilder<'a> {
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

    pub fn asset(&mut self, asset: &Asset) -> &mut Self {
        self.query_params
            .insert(String::from("asset"), asset.to_string());

        self
    }

    pub fn claimant(&mut self, claimant: &str) -> &mut Self {
        self.query_params
            .insert(String::from("claimant"), String::from(claimant));

        self
    }
}

impl<'a> CallBuilder<ClaimableBalance> for ClaimableBalanceCallbuilder<'a> {
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

    fn call(&self) -> Result<Record<ClaimableBalance>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/claimable_balances",
        );

        api_call::<Record<ClaimableBalance>>(
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
    fn test_claimable_balance_sponsor() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

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
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let lsp = Asset::new(
            String::from("LSP"),
            String::from("GAB7STHVD5BDH3EEYXPI3OM7PCS4V443PYB5FNT6CFGJVPDLMKDM24WK"),
        )
        .unwrap();

        let cbcb = ClaimableBalanceCallbuilder::new(&s)
            .asset(&lsp)
            .limit(1)
            .call()
            .unwrap();

        assert_eq!(lsp.to_string(), cbcb._embedded.records[0].asset);
    }
}
