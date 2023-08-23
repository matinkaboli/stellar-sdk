#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

pub struct StellarTomlResolver;

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlDocumentation {
    pub ORG_NAME: Option<String>,
    pub ORG_DBA: Option<String>,
    pub ORG_URL: Option<String>,
    pub ORG_LOGO: Option<String>,
    pub ORG_DESCRIPTION: Option<String>,
    pub ORG_PHYSICAL_ADDRESS: Option<String>,
    pub ORG_PHYSICAL_ADDRESS_ATTESTATION: Option<String>,
    pub ORG_PHONE_NUMBER: Option<String>,
    pub ORG_PHONE_NUMBER_ATTESTATION: Option<String>,
    pub ORG_KEYBASE: Option<String>,
    pub ORG_TWITTER: Option<String>,
    pub ORG_GITHUB: Option<String>,
    pub ORG_OFFICIAL_EMAIL: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlPrincipals {
    pub name: Option<String>,
    pub email: Option<String>,
    pub keybase: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub id_photo_hash: Option<String>,
    pub verification_photo_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlValidators {
    pub ALIAS: Option<String>,
    pub DISPLAY_NAME: Option<String>,
    pub HOST: Option<String>,
    pub PUBLIC_KEY: Option<String>,
    pub HISTORY: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlCurrencies {
    pub code: Option<String>,
    pub issuer: Option<String>,
    pub display_decimals: Option<u8>,
    pub is_asset_anchored: Option<bool>,
    pub anchor_asset_type: Option<String>,
    pub anchor_asset: Option<String>,
    pub redemption_instructions: Option<String>,
    pub collateral_addresses: Option<String>,
    pub collateral_address_signatures: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub conditions: Option<String>,
    pub image: Option<String>,
    pub fixed_number: Option<u32>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StellarToml {
    pub NETWORK_PASSPHRASE: Option<String>,
    pub FEDERATION_SERVER: Option<String>,
    pub SIGNING_REQUEST_ACCOUNT: Option<String>,
    pub DEPOSIT_SERVER: Option<String>,
    pub AUTH_SERVER: Option<String>,
    pub TRANSFER_SERVER: Option<String>,
    pub SIGNING_KEY: Option<String>,
    pub HORIZON_URL: Option<String>,
    pub ACCOUNTS: Option<Vec<String>>,
    pub VERSION: Option<String>,
    pub DOCUMENTATION: Option<TomlDocumentation>,
    pub PRINCIPALS: Option<Vec<TomlPrincipals>>,
    pub CURRENCIES: Option<Vec<TomlCurrencies>>,
}

impl StellarTomlResolver {
    pub fn resolve(domain: &str) -> Result<StellarToml, anyhow::Error> {
        let url = format!("https://{}/.well-known/stellar.toml", domain);

        let res = ureq::get(&url).call()?;
        let res_str = res.into_string()?;

        Ok(toml::from_str::<StellarToml>(&res_str)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_resolve() {
        let rabet_toml = StellarTomlResolver::resolve("rabet.io").unwrap();

        if let Some(d) = rabet_toml.DOCUMENTATION {
            assert_eq!(Some(String::from("Rabet")), d.ORG_DBA);
        }
    }
}
