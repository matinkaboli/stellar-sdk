use anyhow::bail;
use nacl::sign::generate_keypair;

#[derive(Debug)]
struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}

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
    // pub fn from_public_key(public_key: &str) -> Result<Self, anyhow::Error> {
    //     let decoded = str_key::decode_check(&VersionBytes::Ed25519PublicKey, public_key);
    //
    //     if let Err(_) = decoded {
    //         bail!("Invalid Stellar public key")
    //     }
    //
    //     let pk = decoded.unwrap();
    //
    //     if pk.len() != 32 {
    //         bail!("Invalid Stellar public key")
    //     }
    //
    //     Ok(Self {
    //         public_key: pk.unwrap(),
    //         secret_key: None,
    //     })
    // }
    //
    pub fn from_raw_ed25519_seed(seed: &Vec<u8>) -> Result<Keypair, anyhow::Error> {
        Self::new_from_secret_key(seed.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_key;

    #[test]
    fn test_import_from_secret_key() {
        let seed = "SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R";
        let seed_bytes =
            str_key::decode_check(&str_key::VersionBytes::Ed25519SecretSeed, seed).unwrap();

        let keypair = Keypair::from_raw_ed25519_seed(&seed_bytes);

        println!("{:?}", keypair);
    }
}
