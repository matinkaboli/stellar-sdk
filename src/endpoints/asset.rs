#[derive(Debug)]
pub struct Asset<'a>(pub &'a str, pub &'a str, pub bool);

impl<'a> Asset<'a> {
    pub fn new(code: &'a str, issuer: &'a str) -> Self {
        Asset(code, issuer, false)
    }

    pub fn native() -> Self {
        Asset("XLM", "", true)
    }

    pub fn as_str(&self) -> String {
        if self.2 {
            return String::from("XLM");
        }

        format!("{}:{}", self.0, self.1)
    }

    pub fn get_type(&self) -> String {
        if self.2 {
            return String::from("native");
        }

        if self.1.len() <= 4 {
            return String::from("credit_alphanum4");
        }

        String::from("credit_alphanum12")
    }
}

impl<'a> Eq for Asset<'a> {}
impl<'a> PartialEq for Asset<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_as_str() {
        let usdc = Asset::new(
            "USDC",
            "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
        );

        let asset_in_str =
            String::from("USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN");

        assert_eq!(usdc.as_str(), asset_in_str);
    }

    #[test]
    fn test_is_native() {
        let usdc = Asset::new(
            "USDC",
            "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
        );

        let xlm = Asset::native();

        assert_ne!(usdc.2, xlm.2);
    }

    #[test]
    fn test_assets_equal() {
        let usdc = Asset::new(
            "USDC",
            "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
        );

        let aqua = Asset::new(
            "AQUA",
            "GBNZILSTVQZ4R7IKQDGHYGY2QXL5QOFJYQMXPKWRRM5PAV7Y4M67AQUA",
        );

        assert!(usdc != aqua);
    }

    fn test_assets_type() {
        let usdc = Asset::new(
            "USDC",
            "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
        );

        let doget = Asset::new(
            "DOGET",
            "GDOEVDDBU6OBWKL7VHDAOKD77UP4DKHQYKOKJJT5PR3WRDBTX35HUEUX",
        );

        let xlm = Asset::native();

        assert_eq!(xlm.get_type(), "native");
        assert_eq!(usdc.get_type(), "credit_alphanum4");
        assert_eq!(doget.get_type(), "credit_alphanum12");
    }
}
