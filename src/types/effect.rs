use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::{Reserve, ResponseLink};

#[derive(Deserialize, Serialize, Debug)]
pub struct EffectLinks {
    #[serde(rename(serialize = "self", deserialize = "self"))]
    pub operation: Option<ResponseLink>,
    pub precedes: Option<ResponseLink>,
    pub succeeds: Option<ResponseLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Effect {
    _links: EffectLinks,
    id: String,
    paging_token: String,
    account: String,
    r#type: String,
    type_i: u8,
    created_at: String,
    starting_balance: Option<String>,
    asset_type: Option<String>,
    amount: Option<String>,
    weight: Option<u32>,
    public_key: Option<String>,
    key: Option<String>,
    low_threshold: Option<u8>,
    med_threshold: Option<u8>,
    high_threshold: Option<u8>,
    home_domain: Option<String>,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    limit: Option<String>,
    auth_required_flag: Option<bool>,
    auth_revokable_flag: Option<bool>,
    name: Option<bool>,
    value: Option<bool>,
    new_seq: Option<String>,
    liquidity_pool_id: Option<String>,
    balance_type_i: Option<String>,
    asset: Option<String>,
    sponsor: Option<String>,
    new_sponsor: Option<String>,
    former_sponsor: Option<String>,
    data_name: Option<String>,
    signer: Option<String>,
    fee_bp: Option<String>,
    total_trustlines: Option<String>,
    total_shares: Option<String>,
    shares_received: Option<String>,
    shares_redeemed: Option<String>,
    liquidity_pool: Option<String>,
    shares_revoked: Option<String>,
    reserves_revoked: Option<Vec<Reserve>>,
}
