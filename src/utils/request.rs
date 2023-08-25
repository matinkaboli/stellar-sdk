use chrono::prelude::*;
use ureq;

pub fn req(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url).call()?.into_string()?;

    Ok(body)
}

pub fn get_current_server_time(hostname: &str) -> Option<i64> {
    let response = ureq::get(hostname).call();

    let mut server_timestamp: i64 = 0;
    if response.is_ok() {
        let response_unwrapped = response.unwrap();
        let date_server_str = response_unwrapped.header("Date").unwrap();
        if let Ok(chrono_date) = DateTime::parse_from_rfc2822(date_server_str) {
            server_timestamp = chrono_date.timestamp();
        }
    }

    if server_timestamp == 0 {
        return None;
    }

    Some(server_timestamp)
}
