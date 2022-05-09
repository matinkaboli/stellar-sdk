use crate::endpoints::{asset_horizon, Record, Server};
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct AssetCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub endpoint: Endpoint,
}

impl<'a> AssetCallBuilder<'a> {
    pub fn new(s: &'a Server) -> Self {
        AssetCallBuilder {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            asset_code: None,
            asset_issuer: None,
            endpoint: Endpoint::None,
        }
    }

    pub fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.cursor = Some(String::from(cursor));

        self
    }

    pub fn order(&mut self, dir: Direction) -> &mut Self {
        self.order = Some(dir);

        self
    }

    pub fn limit(&mut self, limit_number: u8) -> &mut Self {
        self.limit = Some(limit_number);

        self
    }

    pub fn asset_code(&mut self, code: &str) -> &mut Self {
        self.asset_code = Some(String::from(code));

        self
    }

    pub fn asset_issuer(&mut self, issuer: &str) -> &mut Self {
        self.asset_issuer = Some(String::from(issuer));

        self
    }

    pub fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    pub fn call(&self) -> Result<Record<asset_horizon::AssetHorizon>, &str> {
        let mut url = String::from(format!(
            "{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/assets?",
        ));

        if let Some(x) = &self.cursor {
            let s = format!("&cursor={}", x);
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.order {
            let s = format!("&order={}", x.as_str());
            url.push_str(&s[..]);
        }

        if let Some(x) = &self.limit {
            let s = format!("&limit={}", x);
            url.push_str(&s[..]);
        }

        let resp = req(&url);

        match resp {
            Ok(d) => {
                let p: Record<asset_horizon::AssetHorizon> = serde_json::from_str(&d).unwrap();

                Ok(p)
            }
            Err(_) => Err("Error while fetching data from horizon."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut acb = AssetCallBuilder::new(&s);

        let asset_records = acb.limit(3).call().unwrap();

        assert_eq!(asset_records._embedded.records.len(), 3);
    }
}
