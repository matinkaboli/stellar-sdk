use std::fmt::Display;
use std::{error::Error, fmt::Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HorizonError {
    r#type: String,
    title: String,
    status: i16,
    detail: String,
    extras: Option<ExtraHorizonError>,
}

impl Display for HorizonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Result::Ok(())
    }
}

impl Error for HorizonError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraHorizonError {
    envelope_xdr: String,
    result_codes: ErrorResultCode,
    result_xdr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResultCode {
    transaction: String,
    operations: Vec<String>,
}
