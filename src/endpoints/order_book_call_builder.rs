use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::Server;
use crate::types::{Asset, OrderBook};
use crate::utils::Endpoint;

#[derive(Debug)]
pub struct OrderBookCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> OrderBookCallBuilder<'a> {
    pub fn new(s: &'a Server, selling: Asset, buying: Asset) -> Self {
        let mut new_self = Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        };

        new_self
            .query_params
            .extend(selling.as_querystring_hashmap(String::from("selling")));

        new_self
            .query_params
            .extend(buying.as_querystring_hashmap(String::from("buying")));

        new_self
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params
            .insert(String::from("limit"), limit.to_string());

        self
    }

    pub fn call(&self) -> Result<OrderBook, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/order_book"
        );

        api_call::<OrderBook>(
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
    fn test_order_book() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let native = Asset::native();
        let bat = Asset::new(
            String::from("BAT"),
            String::from("GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR"),
        )
        .unwrap();

        let mut ocb = OrderBookCallBuilder::new(&s, native, bat);

        let op_records = ocb.limit(1).call().unwrap();

        assert_eq!(1, op_records.bids.len());
    }
}
