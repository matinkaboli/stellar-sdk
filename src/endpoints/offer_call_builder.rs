use crate::endpoints::{horizon::Record, CallBuilder, Server};
use crate::types::{Asset, Offer};
use crate::utils::{req, Direction, Endpoint};

#[derive(Debug)]
pub struct OfferCallBuilder<'a> {
    pub server: &'a Server,
    pub cursor: Option<String>,
    pub order: Option<Direction>,
    pub limit: Option<u8>,
    pub buying: Option<&'a Asset<'a>>,
    pub seller: Option<String>,
    pub selling: Option<&'a Asset<'a>>,
    pub sponsor: Option<String>,
    pub endpoint: Endpoint,
}

impl<'a> OfferCallBuilder<'a> {
    pub fn selling(&mut self, asset: &'a Asset) -> &mut Self {
        self.selling = Some(asset);

        self
    }

    pub fn buying(&mut self, asset: &'a Asset) -> &mut Self {
        self.buying = Some(asset);

        self
    }

    pub fn seller(&mut self, seller: &str) -> &mut Self {
        self.seller = Some(String::from(seller));

        self
    }

    pub fn sponsor(&mut self, sponsor: &str) -> &mut Self {
        self.sponsor = Some(String::from(sponsor));

        self
    }
}

impl<'a> CallBuilder<'a, Offer> for OfferCallBuilder<'a> {
    fn new(s: &'a Server) -> Self {
        OfferCallBuilder {
            server: s,
            cursor: None,
            order: None,
            limit: None,
            sponsor: None,
            buying: None,
            seller: None,
            selling: None,
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

    fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    fn for_endpoint(&mut self, endpoint: Endpoint) -> &mut Self {
        self.endpoint = endpoint;

        self
    }

    fn call(&self) -> Result<Record<Offer>, &str> {
        let mut url = format!("{}{}{}", &self.server.0, self.endpoint.as_str(), "/offers?");

        if let Some(x) = &self.cursor {
            url.push_str(&format!("&cursor={}", x));
        }

        if let Some(x) = &self.order {
            url.push_str(&format!("&order={}", x.as_str()));
        }

        if let Some(x) = &self.limit {
            url.push_str(&format!("&limit={}", x));
        }

        if let Some(x) = &self.sponsor {
            url.push_str(&format!("&sponsor={}", x));
        }

        if let Some(x) = &self.seller {
            url.push_str(&format!("&seller={}", x));
        }

        if let Some(x) = &self.selling {
            url.push_str(&format!("&selling={}", x.as_str()));
        }

        if let Some(x) = &self.buying {
            url.push_str(&format!("&buying={}", x.as_str()));
        }

        let resp = req(&url).unwrap();

        let p: Record<Offer> = serde_json::from_str(&resp).unwrap();

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offer_call_builder() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let mut ocb = OfferCallBuilder::new(&s);

        let offer_records = ocb.limit(200).call().unwrap();

        assert_eq!(offer_records._embedded.records.len(), 200);
    }

    #[test]
    fn test_offer_call_builder_selling_buying() {
        let s = Server::new(String::from("https://horizon.stellar.org"));

        let y_xlm = Asset::new(
            "yXLM",
            "GARDNV3Q7YGT4AKSDF25LT32YSCCW4EV22Y2TV3I2PU2MMXJTEDL5T55",
        );
        let xlm = Asset::native();

        let records = OfferCallBuilder::new(&s)
            .selling(&y_xlm)
            .buying(&xlm)
            .limit(2)
            .call()
            .unwrap();

        assert_eq!(
            records._embedded.records[0].selling.asset_code,
            Some(String::from("yXLM"))
        );

        assert_eq!(records._embedded.records[1].buying.asset_type, xlm.as_str());
    }
}
