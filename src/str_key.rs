use std::error::Error;

use anyhow::bail;
use byteorder::{ByteOrder, LittleEndian};
use crc::{Crc, CRC_16_XMODEM};
use data_encoding::BASE32;

#[derive(PartialEq, Eq)]
pub enum VersionBytes {
    Ed25519PublicKey,
    Ed25519SecretSeed,
    Med25519PublicKey, // M
    PreAuthTx,         // T
    Sha256Hash,        // X
    SignedPayload,     // P
}

impl ToString for VersionBytes {
    fn to_string(&self) -> String {
        match self {
            VersionBytes::Ed25519PublicKey => String::from("G"),
            VersionBytes::Ed25519SecretSeed => String::from("S"),
            VersionBytes::Med25519PublicKey => String::from("M"),
            VersionBytes::PreAuthTx => String::from("T"),
            VersionBytes::Sha256Hash => String::from("X"),
            VersionBytes::SignedPayload => String::from("P"),
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
            144 => Ok(VersionBytes::Med25519PublicKey), //  18 << 3 == 144
            96 => Ok(VersionBytes::Ed25519SecretSeed),  //  12 << 3 == 96
            152 => Ok(VersionBytes::PreAuthTx),         //  19 << 3 == 152
            184 => Ok(VersionBytes::Sha256Hash),        //  23 << 3 == 184
            120 => Ok(VersionBytes::SignedPayload),     //  15 << 3 == 120
            _ => bail!("is not supported"),
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

fn encode_check(v: VersionBytes, data: &mut Vec<u8>) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(v.into());
    bytes.append(data);
    let mut checksum = calculate_checksum(&bytes);
    bytes.append(&mut checksum);

    BASE32.encode(&bytes)
}

fn decode_check(v: VersionBytes, encoded_data: &str) -> Result<Vec<u8>, anyhow::Error> {
    let decoded = BASE32.decode(encoded_data.as_bytes())?;
    let version_byte: VersionBytes = VersionBytes::try_from(decoded[0])?;
    let payload = &decoded[..decoded.len() - 2];
    let data = &payload[1..];
    let checksum = &decoded[decoded.len() - 2..];

    if encoded_data != BASE32.encode(&decoded) {
        bail!("invalid encode string")
    }

    if version_byte != v {
        bail!("invalid version byte")
    }

    let expected_checksum = calculate_checksum(payload);
    println!("{:#?}", expected_checksum);
    println!("{:#?}", checksum);
    if expected_checksum != checksum {
        bail!("invalid checksum")
    }

    Ok(data.to_vec())
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
            VersionBytes::Ed25519PublicKey,
            &mut vec![
                91_u8, 49_u8, 118_u8, 218_u8, 79_u8, 232_u8, 118_u8, 216_u8, 114_u8, 82_u8, 9_u8,
                175_u8, 17_u8, 217_u8, 95_u8, 50_u8, 155_u8, 52_u8, 15_u8, 112_u8, 137_u8, 99_u8,
                101_u8, 172_u8, 40_u8, 104_u8, 207_u8, 154_u8, 154_u8, 33_u8, 113_u8, 92_u8,
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
            VersionBytes::Ed25519PublicKey,
            "GBNTC5W2J7UHNWDSKIE26EOZL4ZJWNAPOCEWGZNMFBUM7GU2EFYVZNOL",
        )
        .unwrap();

        assert_eq!(
            vec![
                91_u8, 49_u8, 118_u8, 218_u8, 79_u8, 232_u8, 118_u8, 216_u8, 114_u8, 82_u8, 9_u8,
                175_u8, 17_u8, 217_u8, 95_u8, 50_u8, 155_u8, 52_u8, 15_u8, 112_u8, 137_u8, 99_u8,
                101_u8, 172_u8, 40_u8, 104_u8, 207_u8, 154_u8, 154_u8, 33_u8, 113_u8, 92_u8,
            ],
            decoded_public,
        );
    }
}
