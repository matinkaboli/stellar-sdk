use serde::{Deserialize, Serialize};

use crate::endpoints::records::TemplateLink;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceR {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub asset_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationLinks {
    pub itself: TemplateLink,
    pub transaction: TemplateLink,
    pub effects: TemplateLink,
    pub succeeds: TemplateLink,
    pub precedes: TemplateLink,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: String,
    pub paging_token: String,
    pub transaction_successful: String,
    pub source_account: String,
    pub r#type: String,
    pub type_i: u32,
    pub created_at: String,
    pub transaction_hash: String,
    pub starting_balance: Option<String>,
    pub funder: Option<String>,
    pub account: Option<String>,
    pub asset_type: Option<String>,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub amount: Option<String>,
    pub into: Option<String>,
    pub source_amount: Option<String>,
    pub source_asset_code: Option<String>,
    pub source_asset_issuer: Option<String>,
    pub source_asset_type: Option<String>,
    pub path: Option<Vec<Path>>,
    pub destination_min: Option<String>,
    pub offer_id: Option<String>,
    pub buying_asset_type: Option<String>,
    pub buying_asset_code: Option<String>,
    pub buying_asset_issuer: Option<String>,
    pub price: Option<String>,
    pub price_r: Option<PriceR>,
    pub selling_asset_type: Option<String>,
    pub selling_asset_code: Option<String>,
    pub selling_asset_issuer: Option<String>,
    pub signer_key: Option<String>,
    pub signer_weight: Option<u32>,
    pub master_key_weight: Option<u32>,
    pub low_threshold: Option<u32>,
    pub med_threshold: Option<u32>,
    pub high_threshold: Option<u32>,
    pub home_domain: Option<String>,
    pub set_flags: Option<Vec<u32>>,
    pub set_flags_s: Option<Vec<String>>,
    pub clear_flags: Option<Vec<u32>>,
    pub clear_flags_s: Option<Vec<String>>,
    pub liquidity_pool_id: Option<String>,
    pub trustee: Option<String>,
    pub trustor: Option<String>,
    pub limit: Option<String>,
    pub authorize: Option<bool>,
    pub authorize_to_maintain_liabilities: Option<bool>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub bump_to: Option<String>,
}
/*
  export interface Predicate {
    and?: Predicate[];
    or?: Predicate[];
    not?: Predicate;
    abs_before?: string;
    rel_before?: string;
  }
*/
//export interface Claimant {
//destination: string;
//  predicate: Predicate;
//}

//horizon_api L 541
