use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Trade};
use crate::utils::{Direction, Endpoint, TradeType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TradeCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> TradeCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
    }

    pub fn for_asset_pair(&mut self, base: Asset<'a>, counter: Asset<'a>) -> &mut Self {
        self.query_params
            .insert("base_asset_type", &base.get_type());
        self.query_params.insert("base_asset_code", base.0);
        self.query_params.insert("base_asset_code", base.1);

        self.query_params
            .insert("counter_asset_type", &counter.get_type());
        self.query_params.insert("counter_asset_code", counter.0);
        self.query_params.insert("counter_asset_code", counter.1);

        self
    }

    pub fn for_offer(&mut self, offer_id: &'a str) -> &mut Self {
        self.query_params.insert("offer_id", offer_id);

        self
    }

    pub fn for_type(&mut self, f_type: TradeType) -> &mut Self {
        self.query_params.insert("trade_type", &f_type.as_str());

        self
    }
}

impl<'a> CallBuilder<'a, Trade> for TradeCallBuilder<'a> {
    fn call(&self) -> Result<Record<Trade>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades",
        );

        api_call::<Record<Trade>>(url, crate::types::HttpMethod::GET, self.query_params)
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_trade_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb.limit(200).call().unwrap();

        assert_eq!(records._embedded.records.len(), 200);
    }

    #[test]
    fn test_trade_call_builder_asset_pair() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let y_usdc = Asset::new(
            "yUSDC",
            "GDGTVWSM4MGS4T7Z6W4RPWOCHE2I6RDFCIFZGS3DOA63LWQTRNZNTTFF",
        );
        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb.for_asset_pair(native, y_usdc).limit(2).call().unwrap();

        assert_eq!(records._embedded.records.len(), 2);
    }

    #[test]
    fn test_for_type() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let _records = s.trades().for_type(TradeType::Orderbook).call().unwrap();
    }
}
