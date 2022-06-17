use byteorder::{ByteOrder, LittleEndian};
use crc::{Crc, CRC_16_XMODEM};
use data_encoding::BASE32;
use std::{fmt::format, str::Bytes};

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

impl Into<i32> for VersionBytes {
    fn into(self) -> i32 {
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

fn encode_check(v: VersionBytes, data: &Vec<u8>) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(v.into());
    bytes.append(&mut data.as_bytes().to_vec());

    let mut unencoded: Vec<u8> = Vec::new();

    let crc: Crc<u16> = Crc::<u16>::new(&CRC_16_XMODEM);
    let checksum = crc.checksum(&bytes);
    LittleEndian::write_u16(&mut unencoded, checksum);

    bytes.append(&mut unencoded);

    BASE32.encode(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_bytes_into() {
        let public = VersionBytes::Ed25519PublicKey;
        let public_num: i32 = public.into();

        assert_eq!(6 << 3, public_num);
    }

    #[test]
    fn test_buffer() {
        encode_check(VersionBytes::Ed25519PublicKey, "ABCDE")
    }
}
