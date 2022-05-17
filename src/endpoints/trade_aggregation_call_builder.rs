use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, TradeAggregation};
use crate::utils::req;

#[derive(Debug)]
pub struct TradeAggregationCallBuilder<'a> {
    pub server: &'a Server,
    pub limit: Option<u8>,
    pub base: &'a Asset<'a>,
    pub counter: &'a Asset<'a>,
    pub resolution: String,
}

impl<'a> TradeAggregationCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        base: &'a Asset<'a>,
        counter: &'a Asset<'a>,
        resolution: &'a str,
    ) -> Self {
        Self {
            server: s,
            limit: None,
            base,
            counter,
            resolution: String::from(resolution),
        }
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn call(&self) -> Result<Record<TradeAggregation>, &str> {
        let mut url = format!("{}{}", &self.server.0, "/trade_aggregations?");

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        url.push_str(&self.base.as_querystring(String::from("base")));
        url.push_str(&self.counter.as_querystring(String::from("counter")));
        url.push_str(&format!("&resolution={}", self.resolution));

        let resp = req(&url).unwrap();

        let p: Record<TradeAggregation> = serde_json::from_str(&resp).unwrap();

        Ok(p)
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
