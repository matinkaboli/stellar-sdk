use anyhow::{self, bail};
use serde::Deserialize;
use std::collections::HashMap;
use ureq;

use crate::types::HttpMethod;

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

    let res = req.call()?;
    if res.status() != 200 {
        bail!("failed")
    }

    let res_str = res.into_string()?;

    Ok(serde_json::from_str::<T>(&res_str)?)
}
