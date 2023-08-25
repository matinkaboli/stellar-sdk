use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubmitTransactionResponse {
    hash: String,
    ledger: u64,
    successful: bool,
    envelope_xdr: String,
    result_xdr: String,
    result_meta_xdr: String,
    paging_token: String,
}
