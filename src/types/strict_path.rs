use serde::{Deserialize, Serialize};

use crate::endpoints::horizon::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct StrictPath {
    pub source_asset_type: String,
    pub source_asset_code: Option<String>,
    pub source_asset_issuer: Option<String>,
    pub source_amount: String,
    pub destination_asset_type: String,
    pub destination_asset_code: Option<String>,
    pub destination_asset_issuer: Option<String>,
    pub path: Vec<Path>,
    pub destination_amount: String,
}
