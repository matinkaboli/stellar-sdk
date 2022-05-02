pub struct Asset(String, String, bool);

impl Asset {
    pub fn new(code: &str, issuer: &str) -> Self {
        Asset(code, issuer, false)
    }
}
