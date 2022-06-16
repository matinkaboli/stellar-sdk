use crate::endpoints::Server;
use crate::types::{Asset, OrderBook};
use crate::utils::req;

#[derive(Debug)]
pub struct OrderBookCallBuilder<'a> {
    server: &'a Server,
    limit: Option<u8>,
    selling: &'a Asset<'a>,
    buying: &'a Asset<'a>,
}

impl<'a> OrderBookCallBuilder<'a> {
    pub fn new(s: &'a Server, selling: &'a Asset<'a>, buying: &'a Asset<'a>) -> Self {
        Self {
            server: s,
            limit: None,
            selling,
            buying,
        }
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn call(&self) -> Result<OrderBook, &str> {
        let mut url = format!("{}{}", &self.server.0, "/order_book?");

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        url.push_str(&self.selling.as_querystring(String::from("selling")));
        url.push_str(&self.buying.as_querystring(String::from("buying")));

        let resp = req(&url).unwrap();

        let p: OrderBook = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let bat = Asset::new(
            "BAT",
            "GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR",
        );

        let mut ocb = OrderBookCallBuilder::new(&s, &native, &bat);

        let op_records = ocb.limit(5).call().unwrap();

        assert_eq!(5, op_records.bids.len());
    }
}
