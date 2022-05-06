use ureq;

pub fn req(url: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(url).call()?.into_string()?;

    Ok(body)
}
