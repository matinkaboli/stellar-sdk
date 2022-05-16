use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeDistribution {
    pub max: String,
    pub min: String,
    pub mode: String,
    pub p10: String,
    pub p20: String,
    pub p30: String,
    pub p40: String,
    pub p50: String,
    pub p60: String,
    pub p70: String,
    pub p80: String,
    pub p90: String,
    pub p95: String,
    pub p99: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeStats {
    pub last_ledger: String,
    pub last_ledger_base_fee: String,
    pub ledger_capacity_usage: String,
    pub fee_charged: FeeDistribution,
    pub max_fee: FeeDistribution,
}
