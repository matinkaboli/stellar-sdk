use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, ClaimableBalance};
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct ClaimableBalanceCallbuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub endpoint: Endpoint,
    pub sponsor: Option<String>,
    pub asset: Option<&'a Asset<'a>>,
    pub claimant: Option<String>,
}

impl<'a> ClaimableBalanceCallbuilder<'a> {
    pub fn sponsor(&mut self, sponsor: &str) -> &mut Self {
        self.sponsor = Some(String::from(sponsor));

        self
    }

    pub fn asset(&mut self, asset: &'a Asset<'a>) -> &mut Self {
        self.asset = Some(asset);

        self
    }

    pub fn claimant(&mut self, claimant: &str) -> &mut Self {
        self.claimant = Some(String::from(claimant));

        self
    }
}

impl<'a> CallBuilder<'a, ClaimableBalance> for ClaimableBalanceCallbuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            sponsor: None,
            asset: None,
            claimant: None,
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

    fn call(&self) -> Result<Record<ClaimableBalance>, &str> {
        let mut url = format!(
            "{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/claimable_balances?",
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

        if let Some(x) = &self.sponsor {
            url.push_str(&format!("&sponsor={}", x));
        }

        if let Some(x) = &self.asset {
            url.push_str(&format!("&asset={}", x.as_str()));
        }

        if let Some(x) = &self.claimant {
            url.push_str(&format!("&claimant={}", x));
        }

        let resp = req(&url).unwrap();

        let p: Record<ClaimableBalance> = serde_json::from_str(&resp).unwrap();

        Ok(p)
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
