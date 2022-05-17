use crate::endpoints::{horizon::Record, Server};
use crate::types::{Asset, StrictPath};
use crate::utils::req;

#[derive(Debug)]
pub struct StrictReceiveCallBuilder<'a> {
    pub server: &'a Server,
    pub limit: Option<u8>,
    pub source_account: Option<String>,
    pub source_assets: Option<Vec<&'a Asset<'a>>>,
    pub destination_asset: &'a Asset<'a>,
    pub destination_amount: String,
}

impl<'a> StrictReceiveCallBuilder<'a> {
    pub fn new(
        s: &'a Server,
        source_account: Option<String>,
        source_assets: Option<Vec<&'a Asset<'a>>>,
        destination_asset: &'a Asset<'a>,
        destination_amount: &'a str,
    ) -> Self {
        Self {
            server: s,
            limit: None,
            source_account,
            source_assets,
            destination_asset,
            destination_amount: String::from(destination_amount),
        }
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn call(&self) -> Result<Record<StrictPath>, &str> {
        let mut url = format!("{}{}", &self.server.0, "/paths/strict-receive?");

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.source_account {
            url.push_str(&format!("&source_account={}", x));
        }
        if let Some(x) = &self.source_assets {
            let v: Vec<String> = x.iter().map(|&ast| ast.as_str()).collect();
            let v = v.join(",");

            url.push_str(&format!("&source_assets={}", v));
        }

        url.push_str(&format!("&destination_amount={}", self.destination_amount));
        url.push_str(
            &self
                .destination_asset
                .as_querystring(String::from("destination")),
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
    fn test_strict_receive() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let bat = Asset::new(
            "BAT",
            "GBDEVU63Y6NTHJQQZIKVTC23NWLQVP3WJ2RI2OTSJTNYOIGICST6DUXR",
        );

        let _ocb = StrictReceiveCallBuilder::new(&s, None, Some(vec![&bat]), &native, "20")
            .limit(1)
            .call()
            .unwrap();
    }
}
