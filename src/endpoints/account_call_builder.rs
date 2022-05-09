use crate::endpoints::{call_builder::CallBuilder, Account, Asset, Record, Server};
use crate::utils::{req, Direction};

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub signer: Option<String>,
    pub sponsor: Option<String>,
    pub asset: Option<&'a Asset<'a>>,
    pub liquidity_pool: Option<String>,
}

impl<'a> AccountCallBuilder<'a> {
    pub fn sponsor(&mut self, s: &str) -> &mut Self {
        self.sponsor = Some(s.to_owned());

        self
    }

    pub fn signer(&mut self, s: &str) -> &mut Self {
        self.signer = Some(s.to_owned());

        self
    }

    pub fn liquidity_pool(&mut self, l: &str) -> &mut Self {
        self.liquidity_pool = Some(l.to_owned());

        self
    }

    pub fn asset(&mut self, a: &'a Asset) -> &mut Self {
        self.asset = Some(a);

        self
    }
}

impl<'a> CallBuilder<'a, Account> for AccountCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        AccountCallBuilder {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            asset: None,
            signer: None,
            sponsor: None,
            liquidity_pool: None,
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

    fn call(&self) -> Result<Record<Account>, &str> {
        let mut url = format!("{}{}", &self.server.0, "/accounts?");

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

        if let Some(x) = &self.signer {
            url.push_str(&format!("&signer={}", x));
        }

        if let Some(x) = &self.liquidity_pool {
            url.push_str(&format!("&liquidity_pool={}", x));
        }

        if let Some(x) = &self.asset {
            url.push_str(&format!("&asset={}", x.as_str()));
        }

        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: Record<Account> = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_account_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut acb = AccountCallBuilder::new(&s);

        let account_records = acb
            .liquidity_pool("a468d41d8e9b8f3c7209651608b74b7db7ac9952dcae0cdf24871d1d9c7b0088")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(account_records._embedded.records.len(), 3);
    }
}
