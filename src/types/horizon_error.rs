use std::fmt::{Debug, Display};
use std::{error::Error, fmt::Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HorizonError {
    pub r#type: String,
    pub title: String,
    pub status: i16,
    pub detail: Option<String>,
    pub extras: Option<ExtraHorizonError>,
}

impl Display for HorizonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for HorizonError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraHorizonError {
    pub envelope_xdr: Option<String>,
    pub result_codes: Option<ErrorResultCode>,
    pub result_xdr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResultCode {
    pub transaction: String,
    pub operations: Option<Vec<String>>,
}
