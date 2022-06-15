use std::collections::HashMap;

use crate::api_call::api_call;
use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, StrictPath, StrictPathSource};
use crate::utils::{req, Direction, Endpoint};
use crate::CallBuilder;

#[derive(Debug)]
pub struct StrictSendCallBuilder<'a> {
    server_url: &'a str,
    query_params: HashMap<&'a str, &'a str>,
}

impl<'a> CallBuilder<'a, StrictPath> for StrictSendCallBuilder<'a> {
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
        self
    }

    fn call(&self) -> Result<Record<StrictPath>, anyhow::Error> {
        let mut url = format!("{}{}", &self.server_url, "/paths/strict-send");
        api_call::<Record<StrictPath>>(url, crate::types::HttpMethod::GET, self.query_params)
    }
}

impl<'a> StrictSendCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        destination: &StrictPathSource<'a>,
        source_asset: &'a Asset<'a>,
        source_amount: &'a str,
    ) -> Self {
        let new_self = Self {
            server_url: &s.0,
            query_params: HashMap::new(),
        };

        match destination {
            StrictPathSource::Account(account) => {
                new_self.query_params.insert("destination_account", account)
            }
            StrictPathSource::Assets(assets) => new_self.query_params.insert(
                "destination_assets",
                &assets
                    .into_iter()
                    .map(|asset| asset.as_str())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        };

        new_self
            .query_params
            .extend(source_asset.as_querystring_v2("source_asset".to_string()));

        new_self.query_params.insert("source_amount", source_amount);

        new_self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_send() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let bat = Asset::new(
            "BAT",
            "GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR",
        );

        let _ocb = StrictSendCallBuilder::new(
            &s,
            StrictPathSource::Account("GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR"),
            &native,
            "20",
        )
        .limit(1)
        .call()
        .unwrap();
    }
}
