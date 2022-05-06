#[derive(Debug)]
pub struct Asset<'a>(&'a str, &'a str, bool);

impl<'a> Asset<'a> {
    pub fn new(code: &'a str, issuer: &'a str) -> Self {
        Asset(code, issuer, false)
    }

    pub fn as_str(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }

    pub fn native() -> Self {
        Asset("XLM", "", true)
    }
}

impl<'a> Eq for Asset<'a> {}
impl<'a> PartialEq for Asset<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
