use anyhow::bail;
use byteorder::{ByteOrder, LittleEndian};
use crc::{Crc, CRC_16_XMODEM};
use data_encoding::BASE32;

#[derive(PartialEq, Eq, Clone, Debug)]
enum VersionBytes {
    Ed25519PublicKey,  // G
    Ed25519SecretSeed, // S
    Med25519PublicKey, // M
    PreAuthTx,         // T
    Sha256Hash,        // X
    SignedPayload,     // P
}

impl ToString for VersionBytes {
    fn to_string(&self) -> String {
        match self {
            VersionBytes::Ed25519PublicKey => String::from("ed25519PublicKey"),
            VersionBytes::Ed25519SecretSeed => String::from("ed25519SecretSeed"),
            VersionBytes::Med25519PublicKey => String::from("med25519PublicKey"),
            VersionBytes::PreAuthTx => String::from("preAuthTx"),
            VersionBytes::Sha256Hash => String::from("sha256Hash"),
            VersionBytes::SignedPayload => String::from("signedPayload"),
        }
    }
}

impl Into<u8> for VersionBytes {
    fn into(self) -> u8 {
        match self {
            VersionBytes::Ed25519PublicKey => 6 << 3,
            VersionBytes::Ed25519SecretSeed => 18 << 3,
            VersionBytes::Med25519PublicKey => 12 << 3,
            VersionBytes::PreAuthTx => 19 << 3,
            VersionBytes::Sha256Hash => 23 << 3,
            VersionBytes::SignedPayload => 15 << 3,
        }
    }
}

impl TryFrom<u8> for VersionBytes {
    type Error = anyhow::Error;

    fn try_from(version_bye: u8) -> Result<Self, Self::Error> {
        match version_bye {
            48 => Ok(VersionBytes::Ed25519PublicKey),   //  6 << 3 == 48
            144 => Ok(VersionBytes::Ed25519SecretSeed), //  18 << 3 == 144
            96 => Ok(VersionBytes::Med25519PublicKey),  //  12 << 3 == 96
            152 => Ok(VersionBytes::PreAuthTx),         //  19 << 3 == 152
            184 => Ok(VersionBytes::Sha256Hash),        //  23 << 3 == 184
            120 => Ok(VersionBytes::SignedPayload),     //  15 << 3 == 120
            _ => bail!("invalid version byte"),
        }
    }
}

impl TryFrom<char> for VersionBytes {
    type Error = anyhow::Error;

    fn try_from(version_bye: char) -> Result<Self, Self::Error> {
        match version_bye {
            'G' => Ok(VersionBytes::Ed25519PublicKey),
            'S' => Ok(VersionBytes::Ed25519SecretSeed),
            'M' => Ok(VersionBytes::Med25519PublicKey),
            'T' => Ok(VersionBytes::PreAuthTx),
            'X' => Ok(VersionBytes::Sha256Hash),
            'P' => Ok(VersionBytes::SignedPayload),
            _ => bail!("invalid version byte"),
        }
    }
}

fn calculate_checksum(bytes: &[u8]) -> Vec<u8> {
    let mut unencoded: Vec<u8> = vec![0; 2];
    let crc: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);
    let checksum = crc.checksum(bytes);
    LittleEndian::write_u16(&mut unencoded, checksum);
    unencoded
}

fn encode_check(v: &VersionBytes, data: &[u8]) -> String {
    let mut bytes: Vec<u8> = vec![v.clone().into()];
    bytes.append(&mut data.to_owned());
    let mut checksum = calculate_checksum(&bytes);
    bytes.append(&mut checksum);

    BASE32.encode(&bytes)
}

fn decode_check(v: &VersionBytes, encoded_data: &str) -> Result<Vec<u8>, anyhow::Error> {
    let decoded = BASE32.decode(encoded_data.as_bytes())?;
    let version_byte: VersionBytes = VersionBytes::try_from(decoded[0])?;
    let payload = &decoded[..decoded.len() - 2];
    let data = &payload[1..];
    let checksum = &decoded[decoded.len() - 2..];

    if encoded_data != BASE32.encode(&decoded) {
        bail!("invalid encode string")
    }

    if &version_byte != v {
        bail!("invalid version byte")
    }

    let expected_checksum = calculate_checksum(payload);
    if expected_checksum != checksum {
        bail!("invalid checksum")
    }

    Ok(data.to_vec())
}

fn is_valid(v: &VersionBytes, encoded: &str) -> bool {
    match v {
        VersionBytes::Ed25519PublicKey
        | VersionBytes::Ed25519SecretSeed
        | VersionBytes::PreAuthTx
        | VersionBytes::Sha256Hash => {
            if encoded.len() != 56 {
                return false;
            }
        }
        VersionBytes::Med25519PublicKey => {
            if encoded.len() != 69 {
                return false;
            }
        }
        VersionBytes::SignedPayload => {
            if encoded.len() < 56 || encoded.len() > 165 {
                return false;
            }
        }
    };

    match decode_check(v, encoded) {
        Ok(decoded) => match v {
            VersionBytes::Ed25519PublicKey
            | VersionBytes::Ed25519SecretSeed
            | VersionBytes::PreAuthTx
            | VersionBytes::Sha256Hash => decoded.len() == 32,
            VersionBytes::Med25519PublicKey => decoded.len() == 40,
            VersionBytes::SignedPayload => {
                decoded.len() >= 32 + 4 + 4 && decoded.len() <= 32 + 4 + 64
            }
        },
        Err(_err) => false,
    }
}

pub struct StrKey;

impl StrKey {
    pub fn encode_ed25519_public_key(data: &[u8]) -> String {
        encode_check(&VersionBytes::Ed25519PublicKey, data)
    }

    pub fn decode_ed25519_public_key(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::Ed25519PublicKey, data)
    }

    pub fn is_valid_ed25519_public_key(data: &str) -> bool {
        is_valid(&VersionBytes::Ed25519PublicKey, data)
    }

    pub fn encode_ed25519_secret_seed(data: &[u8]) -> String {
        encode_check(&VersionBytes::Ed25519SecretSeed, data)
    }

    pub fn decode_ed25519_secret_seed(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::Ed25519SecretSeed, data)
    }

    pub fn is_valid_ed25519_secret_seed(data: &str) -> bool {
        is_valid(&VersionBytes::Ed25519SecretSeed, data)
    }

    pub fn encode_med25519_public_key(data: &[u8]) -> String {
        encode_check(&VersionBytes::Med25519PublicKey, data)
    }

    pub fn decode_med25519_public_key(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::Med25519PublicKey, data)
    }

    pub fn is_valid_med25519_public_key(data: &str) -> bool {
        is_valid(&VersionBytes::Med25519PublicKey, data)
    }

    pub fn encode_pre_auth_tx(data: &[u8]) -> String {
        encode_check(&VersionBytes::PreAuthTx, data)
    }

    pub fn decode_pre_auth_tx(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::PreAuthTx, data)
    }

    pub fn is_valid_pre_auth_tx(data: &str) -> bool {
        is_valid(&VersionBytes::PreAuthTx, data)
    }

    pub fn encode_sha256_hash(data: &[u8]) -> String {
        encode_check(&VersionBytes::Sha256Hash, data)
    }

    pub fn decode_sha256_hash(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::Sha256Hash, data)
    }

    pub fn is_valid_sha256_hash(data: &str) -> bool {
        is_valid(&VersionBytes::Sha256Hash, data)
    }

    pub fn encode_signed_payload(data: &[u8]) -> String {
        encode_check(&VersionBytes::SignedPayload, data)
    }

    pub fn decode_signed_payload(data: &str) -> Result<Vec<u8>, anyhow::Error> {
        decode_check(&VersionBytes::SignedPayload, data)
    }

    pub fn is_valid_signed_payload(data: &str) -> bool {
        is_valid(&VersionBytes::SignedPayload, data)
    }

    pub fn get_version_byte_for_prefix(data: &str) -> Result<String, anyhow::Error> {
        let decoded = BASE32.decode(data.as_bytes())?;

        Ok(VersionBytes::try_from(decoded[0])?.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_bytes_into() {
        let public = VersionBytes::Ed25519PublicKey;
        let public_num: u8 = public.into();

        assert_eq!(6 << 3, public_num);
    }

    #[test]
    fn test_encode() {
        let encoded_public = encode_check(
            &VersionBytes::Ed25519PublicKey,
            &mut [
                91u8, 49, 118, 218, 79, 232, 118, 216, 114, 82, 9, 175, 17, 217, 95, 50, 155, 52,
                15, 112, 137, 99, 101, 172, 40, 104, 207, 154, 154, 33, 113, 92,
            ],
        );

        assert_eq!(
            "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOL",
            encoded_public
        );
    }

    #[test]
    fn test_decode() {
        let decoded_public = decode_check(
            &VersionBytes::Ed25519PublicKey,
            "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOL",
        )
        .unwrap();

        assert_eq!(
            vec![
                91u8, 49, 118, 218, 79, 232, 118, 216, 114, 82, 9, 175, 17, 217, 95, 50, 155, 52,
                15, 112, 137, 99, 101, 172, 40, 104, 207, 154, 154, 33, 113, 92,
            ],
            decoded_public,
        );
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid(
            &VersionBytes::Ed25519PublicKey,
            "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOL",
        ));

        assert!(!is_valid(
            &VersionBytes::Ed25519PublicKey,
            "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOB",
        ));
    }

    #[test]
    fn test_get_version_byte_prefix() {
        let my_public_key = "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOB";

        let prefix = StrKey::get_version_byte_for_prefix(my_public_key).unwrap();

        assert_eq!("ed25519PublicKey", prefix)
    }
}
