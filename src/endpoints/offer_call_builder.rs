use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Offer};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct OfferCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> OfferCallBuilder<'a> {
    pub fn selling(&mut self, asset: &'a Asset) -> &mut Self {
        self.query_params.insert("selling", &asset.as_str());

        self
    }

    pub fn buying(&mut self, asset: &'a Asset) -> &mut Self {
        self.query_params.insert("buying", &asset.as_str());

        self
    }

    pub fn seller(&mut self, seller: &'a str) -> &mut Self {
        self.query_params.insert("seller", seller);

        self
    }

    pub fn sponsor(&mut self, sponsor: &'a str) -> &mut Self {
        self.query_params.insert("sponsor", sponsor);

        self
    }
}

impl<'a> CallBuilder<'a, Offer> for OfferCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        OfferCallBuilder {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
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

    fn call(&self) -> Result<Record<Offer>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/offers"
        );

        api_call::<Record<Offer>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut ocb = OfferCallBuilder::new(&s);

        let offer_records = ocb.limit(200).call().unwrap();

        assert_eq!(offer_records._embedded.records.len(), 200);
    }

    #[test]
    fn test_offer_call_builder_selling_buying() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let y_xlm = Asset::new(
            "yXLM",
            "GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55",
        );
        let xlm = Asset::native();

        let records = OfferCallBuilder::new(&s)
            .selling(&y_xlm)
            .buying(&xlm)
            .limit(2)
            .call()
            .unwrap();

        assert_eq!(
            records._embedded.records[0].selling.asset_code,
            Some(String::from("yXLM"))
        );

        assert_eq!(records._embedded.records[1].buying.asset_type, xlm.as_str());
    }
}
