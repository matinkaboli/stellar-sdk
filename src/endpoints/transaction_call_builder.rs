use crate::endpoints::{call_builder::CallBuilder, Record, Server, Transaction};
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

    fn cursor(&mut self, c: &str) -> &mut Self {
        self.cursor = Some(c.to_owned());

        self
    }

    fn order(&mut self, o: Direction) -> &mut Self {
        self.order = Some(o);

        self
    }

    fn limit(&mut self, l: u8) -> &mut Self {
        self.limit = Some(l);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Transaction>, &str> {
        let mut url = String::from(format!(
            "{}{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/transactions?",
            format!("&include_failed={}", self.include_failed),
        ));

        if let Some(x) = &self.cursor {
            let s = format!("&cursor={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.order {
            let s = format!("&order={}", x.as_str());
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.limit {
            let s = format!("&limit={}", x);
            url.push_str(&s[..]);
        }

        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: Record<Transaction> = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
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
            .limit(1)
            .call()
            .unwrap();

        assert_eq!(tx_records._embedded.records.len(), 1);
    }
}
