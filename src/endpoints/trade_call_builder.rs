use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Trade};
use crate::utils::{Direction, Endpoint, TradeType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TradeCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> TradeCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }

    pub fn for_asset_pair(&mut self, base: &Asset, counter: &Asset) -> &mut Self {
        self.query_params
            .extend(base.as_querystring_hashmap(String::from("base")));
        self.query_params
            .extend(counter.as_querystring_hashmap(String::from("counter")));

        self
    }

    pub fn for_offer(&mut self, offer_id: &str) -> &mut Self {
        self.query_params
            .insert(String::from("offer_id"), String::from(offer_id));

        self
    }

    pub fn for_type(&mut self, f_type: TradeType) -> &mut Self {
        self.query_params
            .insert(String::from("trade_type"), f_type.as_str());

        self
    }
}

impl<'a> CallBuilder<Trade> for TradeCallBuilder<'a> {
    fn call(&self) -> Result<Record<Trade>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades",
        );

        api_call::<Record<Trade>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_trade_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb.limit(200).call().unwrap();

        assert_eq!(records._embedded.records.len(), 200);
    }

    #[test]
    fn test_trade_call_builder_asset_pair() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let native = Asset::native();
        let y_usdc = Asset::new(
            String::from("yUSDC"),
            String::from("GDGTVWSM4MGS4T7Z6W4RPWOCHE2I6RDFCIFZGS3DOA63LWQTRNZNTTFF"),
        )
        .unwrap();
        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb
            .for_asset_pair(&native, &y_usdc)
            .limit(2)
            .call()
            .unwrap();

        assert_eq!(records._embedded.records.len(), 2);
    }

    #[test]
    fn test_for_type() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let _records = s.trades().for_type(TradeType::Orderbook).call().unwrap();
    }
}
