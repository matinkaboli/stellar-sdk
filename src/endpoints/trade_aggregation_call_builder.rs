use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, TradeAggregation};
use crate::utils::{Direction, Endpoint};
use crate::CallBuilder;

#[derive(Debug)]
pub struct TradeAggregationCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> TradeAggregationCallBuilder<'a> {
    pub fn new(s: &'a Server, base: &Asset, counter: &Asset, resolution: &str) -> Self {
        let mut new_self = Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        };

        new_self
            .query_params
            .extend(base.as_querystring_hashmap("base".to_string()));
        new_self
            .query_params
            .extend(counter.as_querystring_hashmap("counter".to_string()));
        new_self
            .query_params
            .insert(String::from("resolution"), String::from(resolution));

        new_self
    }
}

impl<'a> CallBuilder<TradeAggregation> for TradeAggregationCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<TradeAggregation>, anyhow::Error> {
        let url = format!("{}{}", &self.server_url, "/trade_aggregations");
        api_call::<Record<TradeAggregation>>(
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
    fn test_trade_aggregation() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let native = Asset::native();
        let bat = Asset::new(
            String::from("BAT"),
            String::from("GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR"),
        )
        .unwrap();

        let mut ocb = TradeAggregationCallBuilder::new(&s, &native, &bat, "60000");

        let op_records = ocb.limit(1).call().unwrap();

        assert_eq!(1, op_records._embedded.records.len());
    }
}
