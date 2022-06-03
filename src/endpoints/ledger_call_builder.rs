use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Ledger;
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct LedgerCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub endpoint: Endpoint,
}

impl<'a> CallBuilder<'a, Ledger> for LedgerCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
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

    fn limit(&mut self, limit_number: u8) -> &mut Self {
        self.limit = Some(limit_number);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Ledger>, &str> {
        let mut url = format!(
            "{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/ledgers?",
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

        let resp = req(&url).unwrap();

        let p: Record<Ledger> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut lcb = LedgerCallBuilder::new();

        let ledger_records = lcb.limit(200).call().unwrap();

        assert_eq!(ledger_records._embedded.records.len(), 200);
    }
}
