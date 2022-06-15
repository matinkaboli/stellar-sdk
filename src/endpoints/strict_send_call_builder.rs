use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, StrictPath};
use crate::utils::req;

#[derive(Debug)]
pub struct StrictSendCallBuilder<'a> {
    pub server: &'a Server,
    pub limit: Option<u8>,
    pub destination_account: Option<String>,
    pub destination_assets: Option<Vec<&'a Asset<'a>>>,
    pub source_asset: &'a Asset<'a>,
    pub source_amount: String,
}

impl<'a> StrictSendCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        destination_account: Option<String>,
        destination_assets: Option<Vec<&'a Asset<'a>>>,
        source_asset: &'a Asset<'a>,
        source_amount: &'a str,
    ) -> Self {
        Self {
            server: s,
            limit: None,
            destination_account,
            destination_assets,
            source_asset,
            source_amount: String::from(source_amount),
        }
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn call(&self) -> Result<Record<StrictPath>, &str> {
        let mut url = format!("{}{}", &self.server.0, "/paths/strict-send?");

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.destination_account {
            url.push_str(&format!("&destination_account={}", x));
        }
        if let Some(x) = &self.destination_assets {
            let v: Vec<String> = x.iter().map(|&ast| ast.as_str()).collect();
            let v = v.join(",");

            url.push_str(&format!("&destination_assets={}", v));
        }

        url.push_str(&format!("&source_amount={}", self.source_amount));
        url.push_str(
            &self
                .source_asset
                .deprecated_as_querystring(String::from("source")),
        );

        let resp = req(&url).unwrap();

        let p: Record<StrictPath> = serde_json::from_str(&resp).unwrap();

        Ok(p)
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

        let _ocb = StrictSendCallBuilder::new(&s, None, Some(vec![&bat]), &native, "20")
            .limit(1)
            .call()
            .unwrap();
    }
}
