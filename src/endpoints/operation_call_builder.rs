use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::Operation;
use crate::utils::{Direction, Endpoint};

#[derive(Debug)]
pub struct OperationCallBuilder<'a> {
    server_url: &'a str,
    endpoint: Endpoint,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> OperationCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        Self {
            server_url: &s.0,
            endpoint: Endpoint::None,
            query_params: HashMap::new(),
        }
    }
}

impl<'a> CallBuilder<'a, Operation> for OperationCallBuilder<'a> {
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

    fn call(&self) -> Result<Record<Operation>, anyhow::Error> {
        let mut url = format!(
            "{}{}{}",
            &self.server_url,
            self.endpoint.as_str(),
            "/operations",
        );

        api_call::<Record<Operation>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

impl<'a> OperationCallBuilder<'a> {
    pub fn include_failed(&mut self, i: bool) -> &mut Self {
        self.query_params.insert("include_failed", &i.to_string());

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_operation_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut ocb = OperationCallBuilder::new(&s);

        let op_records = ocb
            .for_endpoint(Endpoint::Accounts(String::from(
                "GAUZUPTHOMSZEV65VNSRMUDAAE4VBMSRYYAX3UOWYU3BQUZ6OK65NOWM",
            )))
            .limit(200)
            .call()
            .unwrap();

        assert_eq!(op_records._embedded.records.len(), 200);
    }
}
