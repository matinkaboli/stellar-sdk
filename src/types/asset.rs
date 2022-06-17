use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Asset(pub String, pub String, pub bool);

impl Asset {
    pub fn new(code: String, issuer: String) -> Self {
        Asset(code, issuer, false)
    }

    pub fn native() -> Self {
        Asset(String::from("XLM"), String::from(""), true)
    }

    pub fn as_str(&self) -> String {
        if self.2 {
            return String::from("native");
        }

        format!("{}:{}", self.0, self.1)
    }

    pub fn get_type(&self) -> String {
        if self.2 {
            return String::from("native");
        }

        if self.0.len() <= 4 {
            return String::from("credit_alphanum4");
        }

        String::from("credit_alphanum12")
    }

    #[deprecated]
    pub fn as_querystring(&self, name: String) -> String {
        if self.get_type() == "native" {
            return format!("&{}_asset_type={}", name, "native");
        }

        format!(
            "&{}_asset_type={}&{}_asset_code={}&{}_asset_issuer={}",
            name,
            self.get_type(),
            name,
            self.0,
            name,
            self.1,
        )
    }

    pub fn as_querystring_hashmap(&self, name: String) -> HashMap<String, String> {
        let mut query_string = HashMap::<String, String>::new();

        if self.get_type() == "native" {
            query_string.insert(format!("&{}_asset_type", name), String::from("native"));
            return query_string;
        }

        query_string.insert(format!("&{}_asset_type", name), self.get_type());
        query_string.insert(format!("&{}_asset_code", name), self.0.clone());
        query_string.insert(format!("&{}_asset_issuer", name), self.1.clone());
        query_string
    }
}

impl FromStr for Asset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let a = String::from(s);
        let parts = a.split(':').collect::<Vec<&str>>();

        Ok(Self(String::from(parts[0]), String::from(parts[1]), false))
    }
}
impl Eq for Asset {}
impl PartialEq for Asset {
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
            String::from("USDC"),
            String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        );

        let asset_in_str =
            String::from("USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN");

        assert_eq!(usdc.as_str(), asset_in_str);
    }

    #[test]
    fn test_is_native() {
        let usdc = Asset::new(
            String::from("USDC"),
            String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        );

        let xlm = Asset::native();

        assert_ne!(usdc.2, xlm.2);
    }

    #[test]
    fn test_assets_equal() {
        let usdc = Asset::new(
            String::from("USDC"),
            String::from("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
        );

        let aqua = Asset::new(
            String::from("AQUA"),
            String::from("GBNZILSTVQZ4R7IKQDGHYGY2QXL5QOFJYQMXPKWRRM5PAV7Y4M67AQUA"),
        );

        assert!(usdc != aqua);
    }

    #[test]
    fn test_assets_type() {
        let rbt = Asset::new(
            String::from("RBT"),
            String::from("GBSQR5CTKWYQXDGB2KHPB4IZL2FO4KVOWH72WEUSZII7Q32HGGIPSOYS"),
        );

        let doget = Asset::new(
            String::from("DOGET"),
            String::from("GDOEVDDBU6OBWKL7VHDAOKD77UP4DKHQYKOKJJT5PR3WRDBTX35HUEUX"),
        );

        let xlm = Asset::native();

        assert_eq!(xlm.get_type(), "native");
        assert_eq!(rbt.get_type(), "credit_alphanum4");
        assert_eq!(doget.get_type(), "credit_alphanum12");
    }

    #[test]
    fn test_asset_from_str() {
        let asset_str = "VELO:GDM4RQUQQUVSKQA7S6EM7XBZP3FCGH4Q7CL6TABQ7B2BEJ5ERARM2M5M";

        let asset = Asset::from_str(asset_str);

        assert_eq!(asset_str, asset.as_str());
    }
}
