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
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> TradeAggregationCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        base: &'a Asset<'a>,
        counter: &'a Asset<'a>,
        resolution: &'a str,
    ) -> Self {
        let new_self = Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        };

        new_self
            .query_params
            .extend(base.as_querystring_v2("base".to_string()));
        new_self
            .query_params
            .extend(counter.as_querystring_v2("counter".to_string()));
        new_self.query_params.insert("resolution", resolution);

        new_self
    }
}

impl<'a> CallBuilder<'a, TradeAggregation> for TradeAggregationCallBuilder<'a> {
    fn cursor(&mut self, cursor: &'a str) -> &mut Self {
        self.query_params.insert("cursor", cursor);

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.query_params.insert("order", dir.as_str());

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.query_params.insert("limit", &limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<TradeAggregation>, anyhow::Error> {
        let mut url = format!("{}{}", &self.server_url, "/trade_aggregations");
        api_call::<Record<TradeAggregation>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_aggregation() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let bat = Asset::new(
            "BAT",
            "GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR",
        );

        let mut ocb = TradeAggregationCallBuilder::new(&s, &native, &bat, "60000");

        let op_records = ocb.limit(1).call().unwrap();

        assert_eq!(1, op_records._embedded.records.len());
    }
}
