use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{PriceRShortHand, ResponseLink};

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub itself: ResponseLink,
    pub base: ResponseLink,
    pub counter: ResponseLink,
    pub operation: ResponseLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub _links: TradeLinks,
    pub id: String,
    pub paging_token: String,
    pub ledger_close_time: String,
    pub trade_type: String,
    pub base_offer_id: String,
    pub base_account: String,
    pub base_amount: String,
    pub base_asset_type: String,
    pub base_asset_code: Option<String>,
    pub base_asset_issuer: Option<String>,
    pub counter_offer_id: String,
    pub counter_account: String,
    pub counter_amount: String,
    pub counter_asset_type: String,
    pub counter_asset_code: Option<String>,
    pub counter_asset_issuer: Option<String>,
    pub base_is_stellar: Option<bool>,
    pub price: PriceRShortHand<String>,
}
