use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Trade};
use crate::utils::{req, Direction, Endpoint, TradeType};

#[derive(Debug)]
pub struct TradeCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub endpoint: Endpoint,
    pub asset_pair: Option<(Asset<'a>, Asset<'a>)>,
    pub offer: Option<String>,
    pub for_type: Option<TradeType>,
}

impl<'a> TradeCallBuilder<'a> {
    pub fn for_asset_pair(&mut self, asset_pair: (Asset<'a>, Asset<'a>)) -> &mut Self {
        self.asset_pair = Some(asset_pair);

        self
    }

    pub fn for_offer(&mut self, offer_id: &str) -> &mut Self {
        self.offer = Some(String::from(offer_id));

        self
    }

    pub fn for_type(&mut self, f_type: TradeType) -> &mut Self {
        self.for_type = Some(f_type);

        self
    }
}

impl<'a> CallBuilder<'a, Trade> for TradeCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        Self {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            endpoint: Endpoint::None,
            asset_pair: None,
            offer: None,
            for_type: None,
        }
    }

    fn cursor(&mut self, cursor: &str) -> &mut Self {
        self.cursor = Some(String::from(cursor));

        self
    }

    fn order(&mut self, dir: Direction) -> &mut Self {
        self.order = Some(dir);

        self
    }

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Trade>, &str> {
        let mut url = format!("{}{}{}", &self.server.0, self.endpoint.as_str(), "/trades?",);

        if let Some(x) = &self.cursor {
            url.push_str(&format!("&cursor={}", x));
        }

        if let Some(x) = &self.order {
            url.push_str(&format!("&order={}", x.as_str()));
        }

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.asset_pair {
            url.push_str(&format!(
                "{}{}",
                x.0.as_querystring(String::from("base")),
                x.1.as_querystring(String::from("counter"))
            ));
        }

        if let Some(x) = &self.offer {
            url.push_str(&format!("&offer_id={}", x));
        }

        if let Some(x) = &self.for_type {
            url.push_str(&format!("&trade_type={}", x.as_str()))
        }

        let resp = req(&url).unwrap();

        let p: Record<Trade> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_trade_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb.limit(200).call().unwrap();

        assert_eq!(records._embedded.records.len(), 200);
    }

    #[test]
    fn test_trade_call_builder_asset_pair() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let native = Asset::native();
        let y_usdc = Asset::new(
            "yUSDC",
            "GDGTVWSM4MGS4T7Z6W4RPWOCHE2I6RDFCIFZGS3DOA63LWQTRNZNTTFF",
        );
        let mut tcb = TradeCallBuilder::new(&s);

        let records = tcb
            .for_asset_pair((native, y_usdc))
            .limit(2)
            .call()
            .unwrap();

        assert_eq!(records._embedded.records.len(), 2);
    }

    #[test]
    fn test_for_type() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let records = s.trades().for_type(TradeType::Orderbook).call().unwrap();
    }
}
