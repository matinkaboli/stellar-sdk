use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, StrictPath, StrictPathSource};
use crate::utils::{Direction, Endpoint};
use crate::CallBuilder;

#[derive(Debug)]
pub struct StrictSendCallBuilder<'a> {
    server_url: &'a str,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> CallBuilder<StrictPath> for StrictSendCallBuilder<'a> {
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

    fn for_endpoint(&mut self, _endpoint: Endpoint) -> &mut Self {
        self
    }

    fn call(&self) -> Result<Record<StrictPath>, anyhow::Error> {
        let url = format!("{}{}", &self.server_url, "/paths/strict-send");
        api_call::<Record<StrictPath>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }
}

impl<'a> StrictSendCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        destination: &StrictPathSource,
        source_asset: &Asset,
        source_amount: &str,
    ) -> Self {
        let mut new_self = Self {
            server_url: &s.server_url,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        };

        match destination {
            StrictPathSource::Account(account) => new_self
                .query_params
                .insert(String::from("destination_account"), String::from(account)),

            StrictPathSource::Assets(assets) => new_self.query_params.insert(
                String::from("destination_assets"),
                assets
                    .iter()
                    .map(|asset| asset.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        };

        new_self
            .query_params
            .extend(source_asset.as_querystring_hashmap(String::from("source")));

        new_self
            .query_params
            .insert(String::from("source_amount"), String::from(source_amount));

        new_self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_send() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let native = Asset::native();
        let rbt_issuer = String::from("GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR");

        let _ocb =
            StrictSendCallBuilder::new(&s, &StrictPathSource::Account(rbt_issuer), &native, "20")
                .limit(1)
                .call()
                .unwrap();
    }
}
