use crate::endpoints::{Server, Transaction};

#[derive(Debug)]
pub struct AccountCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Order>,
    pub limit: Option<u32>,
    pub include_failed: bool,
    pub for_ledger: Option<String>,
    pub for_account: Option<String>,
    pub for_transaction: Option<String>,
    pub for_liquidity_pool: Option<String>,
    pub for_claimable_balance: Option<String>,
}

impl AccountCallBuilder {
    pub fn new(s: &'a Server) -> Self {
        AccountCallBuilder {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            include_failed: false,
            for_ledger: None,
            for_account: None,
            for_transaction: None,
            for_liquidity_pool: None,
            for_claimable_balance: None,
        }
    }

    pub fn cursor(&mut self, c: &str) -> &mut Self {
        self.cursor = Some(c.to_owned());

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

    pub fn for_account(&mut self, a: String) -> &mut Self {
        self.for_account = Some(a);

        self
    }

    pub fn for_claimable_balance(&mut self, c: String) -> &mut Self {
        self.for_claimable_balance = Some(c);

        self
    }

    pub fn for_ledger(&mut self, l: String) -> &mut Self {
        self.for_ledger = Some(l);

        self
    }

    pub fn for_liquidity_pool(&mut self, l: String) -> &mut Self {
        self.for_liquidity_pool = Some(l);

        self
    }

    pub fn for_transaction(&mut self, t: String) -> &mut Self {
        self.transaction = Some(t);

        self
    }

    pub fn include_failed(&mut self, i: bool) -> &mut Self {
        self.include_failed = i;

        self
    }

    pub fn call(&self) -> Result<Transaction, &str> {
        let mut url = String::from(&self.server.0);

        url.push_str("/transactions?");

        let i = format!("&include_failed={}", include_failed);

        url.push_str(&i[..]);

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

        if let Some(x) = &self.for_ledger {
            let s = format!("&ledger={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.for_account {
            let s = format!("&account={}", x.as_str());
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.for_claimable_balance {
            let s = format!("&claimable_balance={}", x.as_str());
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
