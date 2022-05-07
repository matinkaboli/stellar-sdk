use crate::endpoints::{Account, Asset, Record, Server};
use crate::utils::{direction, req};

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub sponsor: Option<String>,
    pub order: Option<Order>,
    pub limit: Option<u32>,
    pub signer: Option<String>,
    pub liquidity_pool: Option<String>,
    pub asset: Option<&'a Asset<'a>>,
}

impl<'a> AccountCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        AccountCallBuilder {
            server: s,
            cursor: None,
            order: None,
            sponsor: None,
            limit: None,
            signer: None,
            liquidity_pool: None,
            asset: None,
        }
    }

    pub fn cursor(&mut self, c: &str) -> &mut Self {
        self.cursor = Some(c.to_owned());

        self
    }

    pub fn sponsor(&mut self, s: &str) -> &mut Self {
        self.sponsor = Some(s.to_owned());

        self
    }

    pub fn order(&mut self, o: Order) -> &mut Self {
        self.order = Some(o);

        self
    }

    pub fn limit(&mut self, l: u32) -> &mut Self {
        self.limit = Some(l);

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

    pub fn call(&self) -> Result<Record<Account>, &str> {
        let mut url = String::from(&self.server.0);

        url.push_str("/accounts?");

        if let Some(x) = &self.cursor {
            let s = format!("&cursor={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.sponsor {
            let s = format!("&sponsor={}", x);
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

        if let Some(x) = &self.signer {
            let s = format!("&signer={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.liquidity_pool {
            let s = format!("&liquidity_pool={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.asset {
            let s = format!("&asset={}", x.as_str());
            url.push_str(&s[..]);
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
