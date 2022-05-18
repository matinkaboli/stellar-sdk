use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Transaction;
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct TransactionCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub include_failed: bool,
    pub endpoint: Endpoint,
}

impl<'a> CallBuilder<'a, Transaction> for TransactionCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            include_failed: false,
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

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Transaction>, &str> {
        let mut url = format!(
            "{}{}{}&include_failed={}",
            &self.server.0,
            self.endpoint.as_str(),
            "/transactions?",
            self.include_failed,
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

        let p: Record<Transaction> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

impl<'a> TransactionCallBuilder<'a> {
    pub fn include_failed(&mut self, i: bool) -> &mut Self {
        self.include_failed = i;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_transaction_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut tcb = TransactionCallBuilder::new(&s);

        let tx_records = tcb
            .for_endpoint(Endpoint::Accounts(String::from(
                "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
            )))
            .limit(200)
            .call()
            .unwrap();

        assert_eq!(tx_records._embedded.records.len(), 200);
    }
}
