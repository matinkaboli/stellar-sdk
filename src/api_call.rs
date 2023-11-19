use serde::de::DeserializeOwned;
use std::collections::HashMap;
use ureq::{self, Error as UreqError};

use crate::types::{HorizonError, HttpMethod};

pub fn api_call<T: DeserializeOwned>(
    url: String,
    method: HttpMethod,
    query_params: &HashMap<String, String>,
    token: &Option<String>,
) -> Result<T, anyhow::Error> {
    let mut req = match method {
        HttpMethod::GET => ureq::get(&url),
        HttpMethod::POST => ureq::post(&url),
    };
    if token.is_some() {
        req = req.set("Authorization", token.clone().unwrap().as_str());
    }

    for query_param in query_params.iter() {
        req = req.query(query_param.0, query_param.1);
    }

    match req.call() {
        Ok(res) => {
            let res_str = res.into_string()?;

            Ok(serde_json::from_str::<T>(&res_str)?)
        }
        Err(e) => match e {
            UreqError::Status(code, res) => {
                if code >= 500 {
                    return Err(UreqError::Status(code, res).into());
                }

                let res_str = res.into_string()?;
                let parsed: HorizonError = serde_json::from_str(&res_str)?;

                Err(parsed.into())
            }
            other => Err(other.into()),
        },
    }
}
