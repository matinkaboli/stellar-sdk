use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Effect;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct EffectCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> CallBuilder<'a, Effect> for EffectCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
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
        self.query_params.insert("order", &limit.to_string());

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Effect>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/trades?",
        );

        api_call::<Record<Effect>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut ecb = EffectCallBuilder::new(&s);

        let effect_records = ecb.limit(200).call().unwrap();

        assert_eq!(effect_records._embedded.records.len(), 200);
    }
}
