use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, StrictPath, StrictPathSource};
use crate::utils::{Direction, Endpoint};
use crate::CallBuilder;

#[derive(Debug)]
pub struct StrictReceiveCallBuilder<'a> {
    server_url: &'a str,
    query_params: HashMap<String, String>,
    token: &'a Option<String>,
}

impl<'a> CallBuilder<StrictPath> for StrictReceiveCallBuilder<'a> {
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
        let url = format!("{}{}", &self.server_url, "/paths/strict-receive");
        api_call::<Record<StrictPath>>(
            url,
            crate::types::HttpMethod::GET,
            &self.query_params,
            self.token,
        )
    }
}

impl<'a> StrictReceiveCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        source: &StrictPathSource,
        destination_asset: &Asset,
        destination_amount: &str,
    ) -> Self {
        let mut new_self = Self {
            server_url: &s.server_url,
            query_params: HashMap::new(),
            token: &s.options.auth_token,
        };

        match source {
            StrictPathSource::Account(account) => new_self
                .query_params
                .insert(String::from("source_account"), String::from(account)),
            StrictPathSource::Assets(assets) => new_self.query_params.insert(
                String::from("source_assets"),
                assets
                    .iter()
                    .map(|asset| asset.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        };

        new_self
            .query_params
            .extend(destination_asset.as_querystring_hashmap("destination".to_string()));

        new_self.query_params.insert(
            String::from("destination_amount"),
            String::from(destination_amount),
        );

        new_self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_receive() {
        let s = Server::new(String::from("https://horizon.stellar.org"), None)
            .expect("Cannot connect to insecure horizon server");

        let native = Asset::native();
        let _bat = Asset::new(
            String::from("BAT"),
            String::from("GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR"),
        );

        let _ocb = StrictReceiveCallBuilder::new(
            &s,
            &StrictPathSource::Account(
                "GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR".to_string(),
            ),
            &native,
            "20",
        )
        .limit(1)
        .call()
        .unwrap();
    }
}
