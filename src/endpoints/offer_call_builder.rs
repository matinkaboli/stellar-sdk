use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Offer};
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct OfferCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> OfferCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        OfferCallBuilder {
            server_url: &s.server_url,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        }
    }

    pub fn selling(&mut self, asset: &Asset) -> &mut Self {
        self.query_params
            .insert(String::from("selling"), asset.to_string());

        self
    }

    pub fn buying(&mut self, asset: &Asset) -> &mut Self {
        self.query_params
            .insert(String::from("buying"), asset.to_string());

        self
    }

    pub fn seller(&mut self, seller: &str) -> &mut Self {
        self.query_params
            .insert(String::from("seller"), String::from(seller));

        self
    }

    pub fn sponsor(&mut self, sponsor: &str) -> &mut Self {
        self.query_params
            .insert(String::from("sponsor"), String::from(sponsor));

        self
    }
}

impl<'a> CallBuilder<Offer> for OfferCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<Offer>, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/offers"
        );

        api_call::<Record<Offer>>(
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
    fn test_offer_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let mut ocb = OfferCallBuilder::new(&s);

        let offer_records = ocb.limit(200).call().unwrap();

        assert_eq!(offer_records._embedded.records.len(), 200);
    }

    #[test]
    fn test_offer_call_builder_selling_buying() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let y_xlm = Asset::new(
            String::from("yXLM"),
            String::from("GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55"),
        )
        .unwrap();
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

        assert_eq!(
            records._embedded.records[1].buying.asset_type,
            xlm.to_string()
        );
    }
}
