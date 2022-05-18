use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::AssetHorizon;
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
    pub fn asset_code(&mut self, code: &str) -> &mut Self {
        self.asset_code = Some(String::from(code));

        self
    }

    pub fn asset_issuer(&mut self, issuer: &str) -> &mut Self {
        self.asset_issuer = Some(String::from(issuer));

        self
    }
}

impl<'a> CallBuilder<'a, AssetHorizon> for AssetCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
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

    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.cursor = Some(String::from(cursor));

        self
    }

    fn order(&mut self, o: Direction) -> &mut Self {
        self.order = Some(o);

        self
    }

    fn limit(&mut self, limit_number: u8) -> &mut Self {
        self.limit = Some(limit_number);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<AssetHorizon>, &str> {
        let mut url = format!(
            "{}{}{}",
            &self.server.0,
            self.endpoint.as_str(),
            "/assets?",
        );

        if let Some(x) = &self.cursor {
            url.push_str(&format!("&cursor={}", x));
        }

        if let Some(x) = &self.order {
            url.push_str(&format!("&order={}", x.as_str()));
        }

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.asset_code {
            url.push_str(&format!("&asset_code={}", x));
        }

        if let Some(x) = &self.asset_issuer {
            url.push_str(&format!("&asset_issuer={}", x));
        }

        let resp = req(&url).unwrap();

        let p: Record<AssetHorizon> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_horizon_test() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut acb = AssetCallBuilder::new(&s);

        let asset_records = acb
            .asset_issuer("GA5BUT4SND34VRUJGFEVLG6LMEYOU5HSSYZLX673I2IJVRLLPATMH4RN")
            .limit(3)
            .call()
            .unwrap();

        assert_eq!(asset_records._embedded.records.len(), 3);
    }
}
