use anyhow::bail;
use nacl::sign::{generate_keypair, signature, verify};
use stellar_base::crypto::{SecretKey, SodiumKeyPair};
use str_key::StrKey;

use crate::str_key;

#[derive(Debug, Clone, PartialEq)]
pub struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Keypair {
    fn new_from_secret_key(secret_seed: Vec<u8>) -> Result<Self, anyhow::Error> {
        if secret_seed.len() != 32 {
            bail!("secret_key length is invalid")
        }

        let mut cloned_secret_key = secret_seed.clone();
        let keypair = generate_keypair(&secret_seed);
        let mut pk = keypair.pkey.clone().to_vec();

        let mut secret_key = Vec::new();
        secret_key.append(&mut cloned_secret_key);
        secret_key.append(&mut pk);

        Ok(Self {
            secret_seed: Some(secret_seed),
            public_key: keypair.pkey.to_vec(),
            secret_key: Some(secret_key),
        })
    }

    fn new_from_public_key(public_key: Vec<u8>) -> Result<Self, anyhow::Error> {
        if public_key.len() != 32 {
            bail!("public_key length is invalid")
        }

        Ok(Self {
            public_key,
            secret_key: None,
            secret_seed: None,
        })
    }

    pub fn from_secret_key(secret: &str) -> Result<Self, anyhow::Error> {
        let raw_secret = StrKey::decode_ed25519_secret_seed(secret)?;

        Keypair::from_raw_ed25519_seed(&raw_secret)
    }

    pub fn from_public_key(public_key: &str) -> Result<Self, anyhow::Error> {
        let decoded = StrKey::decode_ed25519_public_key(public_key)?;
        // let decoded = decode_check(&VersionBytes::Ed25519PublicKey, public_key);

        if decoded.len() != 32 {
            bail!("Invalid Stellar public key")
        }

        Ok(Self {
            public_key: decoded,
            secret_seed: None,
            secret_key: None,
        })
    }

    pub fn from_raw_ed25519_seed(seed: &[u8]) -> Result<Self, anyhow::Error> {
        Self::new_from_secret_key(seed.to_vec())
    }

    pub fn raw_secret_key(&self) -> Option<Vec<u8>> {
        self.secret_seed.clone()
    }

    pub fn raw_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn secret_key(&mut self) -> Result<String, anyhow::Error> {
        match &mut self.secret_seed {
            None => bail!("no secret_key available"),
            Some(s) => Ok(StrKey::encode_ed25519_secret_seed(s)),
        }
    }

    pub fn public_key(&self) -> String {
        StrKey::encode_ed25519_public_key(&self.public_key)
    }

    pub fn can_sign(&self) -> bool {
        self.secret_key.is_some()
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        if !self.can_sign() {
            bail!("cannot sign, no secret_key available")
        }

        if let Some(s) = &self.secret_key {
            match signature(data, s) {
                Err(_) => bail!("error while signing"),
                Ok(m) => return Ok(m),
            }
        }

        bail!("error while signing")
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        verify(signature, data, &self.public_key).is_ok()
    }

    pub fn random() -> Result<Self, anyhow::Error> {
        Self::new_from_secret_key(rand::random::<[u8; 32]>().to_vec())
    }
}

// Easy conversion between stellar_sdk and stellar_base keypairs and vica versa
impl From<Keypair> for SodiumKeyPair {
    fn from(sdk_keypair: Keypair) -> Self {
        let mut sdk_keypair_inner = sdk_keypair;
        let secret_key: String = sdk_keypair_inner
            .secret_key()
            .expect("Failed to get the secret key from the stellar_sdk::Keypair");
        let stellar_base_keypair = SodiumKeyPair::from_secret_seed(&secret_key)
            .expect("Failed to convert to generate stellar_base::KeyPair from the secret seed");
        stellar_base_keypair
    }
}

impl From<SodiumKeyPair> for Keypair {
    fn from(base_keypair: SodiumKeyPair) -> Self {
        let secret_key_struct: &SecretKey = base_keypair.secret_key();
        let secret_key: String = secret_key_struct.secret_seed().into();
        let stellar_sdk_keypair = Keypair::from_secret_key(&secret_key)
            .expect("Failed to convert to generate stellar_sdk::Keypair from the secret seed");
        stellar_sdk_keypair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_secret_key() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let pk = String::from("GACAMF2WHKKQTYVHVA3CRMVUHN6GUBLTB7PBJQF73N7ATCIYAIFUCT6B");

        let mut keypair = Keypair::from_secret_key(&seed).unwrap();
        let seed_from_keypair = keypair.secret_key().unwrap();

        assert_eq!(pk, keypair.public_key());
        assert_eq!(seed, seed_from_keypair);
    }

    #[test]
    fn test_can_sign() {
        let pk = String::from("GACAMF2WHKKQTYVHVA3CRMVUHN6GUBLTB7PBJQF73N7ATCIYAIFUCT6B");
        let keypair = Keypair::from_public_key(&pk).unwrap();
        assert!(!keypair.can_sign());

        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();
        assert!(keypair.can_sign());
    }

    #[test]
    fn test_from_raw_seed() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let raw_seed = StrKey::decode_ed25519_secret_seed(&seed).unwrap();

        let keypair = Keypair::from_raw_ed25519_seed(&raw_seed).unwrap();

        if let Some(x) = keypair.raw_secret_key() {
            assert_eq!(raw_seed, x);
        }
    }

    #[test]
    fn test_sign_message() {
        let message = "Hello World".as_bytes().to_vec();

        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();

        let signed_message = keypair.sign(&message).unwrap();

        let expected_signed_message = vec![
            249, 89, 99, 12, 220, 144, 11, 209, 11, 54, 119, 152, 58, 242, 131, 31, 212, 173, 213,
            95, 209, 35, 15, 223, 110, 215, 31, 220, 59, 125, 147, 141, 99, 116, 156, 12, 50, 28,
            137, 31, 0, 175, 86, 235, 92, 157, 151, 132, 88, 222, 147, 50, 248, 15, 191, 208, 153,
            16, 41, 169, 20, 202, 137, 15,
        ];

        assert_eq!(expected_signed_message, signed_message);
    }

    #[test]
    fn test_verify_signed_message() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();

        let unsigned_message = "Hello World".as_bytes().to_vec();
        let signed_message = vec![
            249, 89, 99, 12, 220, 144, 11, 209, 11, 54, 119, 152, 58, 242, 131, 31, 212, 173, 213,
            95, 209, 35, 15, 223, 110, 215, 31, 220, 59, 125, 147, 141, 99, 116, 156, 12, 50, 28,
            137, 31, 0, 175, 86, 235, 92, 157, 151, 132, 88, 222, 147, 50, 248, 15, 191, 208, 153,
            16, 41, 169, 20, 202, 137, 15,
        ];

        assert!(keypair.verify(&signed_message, &unsigned_message))
    }

    #[test]
    fn test_random_keypair() {
        let keypair_1 = Keypair::random().unwrap();
        let keypair_2 = Keypair::random().unwrap();
        let keypair_3 = Keypair::random().unwrap();

        assert_ne!(keypair_1.raw_secret_key(), keypair_2.raw_secret_key());
        assert_ne!(keypair_2.raw_secret_key(), keypair_3.raw_secret_key());
    }

    #[test]
    fn test_keypair_conversion() {
        let keypair_sdk = Keypair::random().unwrap();
        let keypair_base: SodiumKeyPair = keypair_sdk.clone().into();
        let keypair_sdk2: Keypair = keypair_base.into();

        assert_eq!(keypair_sdk, keypair_sdk2);
    }
}
