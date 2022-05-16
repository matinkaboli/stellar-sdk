use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{PriceRShortHand, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub offer_maker: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferAsset {
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    pub id: String,
    pub paging_token: String,
    pub seller: String,
    pub selling: OfferAsset,
    pub buying: OfferAsset,
    pub amount: String,
    pub price_r: PriceRShortHand<u32>,
    pub price: String,
    pub last_modified_ledger: u64,
    pub last_modified_time: String,
    pub sponsor: Option<String>,
}
