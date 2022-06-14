use anyhow::{self, bail};
use serde::Deserialize;
use std::collections::HashMap;
use ureq::{self, Error as UreqError};

use crate::types::{HorizonError, HttpMethod};

pub fn api_call<'a, T: Deserialize<'a>>(
    url: String,
    method: HttpMethod,
    query_params: HashMap<&str, &str>,
) -> Result<T, anyhow::Error> {
    let req: ureq::Request;

    match method {
        HttpMethod::GET => req = ureq::get(&url),
        HttpMethod::POST => req = ureq::post(&url),
    };

    for query_param in query_params.iter() {
        req.query(query_param.0, query_param.1);
    }

    match req.call() {
        Ok(res) => {
            let res_str = res.into_string()?;
            return Ok(serde_json::from_str::<T>(&res_str)?);
        }
        Err(e) => match e {
            UreqError::Status(code, res) => {
                if code >= 500 {
                    bail!("failed");
                }

                let res_str = res.into_string()?;
                let parsed: HorizonError = serde_json::from_str(&res_str)?;

                return Err(parsed.into());
            }
            _ => bail!("failed"),
        },
    }
}