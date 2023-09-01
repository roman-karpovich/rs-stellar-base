use arrayref::array_ref;
use std::str::FromStr;
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use stellar_strkey::Strkey::MuxedAccountEd25519;
use stellar_xdr::*;

use crate::muxed_account;

pub fn decode_address_to_muxed_account(address: &str) -> MuxedAccount {
    if MuxedAccount::from_str(address).is_ok() {
        decode_address_fully_to_muxed_account(address);
    }

    MuxedAccount::from_string(address).unwrap()
}

pub fn encode_muxed_account(address: &str, id: &str) -> stellar_xdr::MuxedAccount {
    let key = PublicKey::from_string(address);

    if key.is_err() {
        panic!("address should be a Stellar account ID (G...)");
    }
    if id.parse::<u64>().is_err() {
        panic!("id should be a string representing a number (uint64)");
    }

    let vv = key.clone().unwrap().0;

    stellar_xdr::MuxedAccount::MuxedEd25519(stellar_xdr::MuxedAccountMed25519 {
        id: id.parse::<u64>().unwrap(),
        ed25519: Uint256(*array_ref!(vv, 0, 32)),
    })
}

pub fn encode_muxed_account_to_address(muxed_account: &stellar_xdr::MuxedAccount) -> String {
    if muxed_account.discriminant() == stellar_xdr::CryptoKeyType::MuxedEd25519 {
        return _encode_muxed_account_fully_to_address(muxed_account);
    }

    let inner_value = match muxed_account {
        stellar_xdr::MuxedAccount::Ed25519(inner) => inner,
        _ => panic!("Expected Ed25519 variant"),
    };

    PublicKey::from_payload(&inner_value.0).unwrap().to_string()
}
pub fn decode_address_fully_to_muxed_account(address: &str) -> stellar_xdr::MuxedAccount {
    let binding = MuxedAccount::from_str(address).unwrap();
    let id = Uint64::from_str(&binding.id.to_string()).unwrap();
    let key = binding.ed25519;
    stellar_xdr::MuxedAccount::MuxedEd25519(stellar_xdr::MuxedAccountMed25519 {
        id,
        ed25519: Uint256(*array_ref!(key, 0, 32)),
    })
}

pub fn _encode_muxed_account_fully_to_address(muxed_account: &stellar_xdr::MuxedAccount) -> String {
    if muxed_account.discriminant() == stellar_xdr::CryptoKeyType::Ed25519 {
        return encode_muxed_account_to_address(muxed_account);
    }

    let inner_value = match muxed_account {
        stellar_xdr::MuxedAccount::MuxedEd25519(inner) => inner,
        _ => panic!("Expected Ed25519 variant"),
    };

    let key = &inner_value.ed25519.0;

    let muxed_account = MuxedAccount {
        ed25519: inner_value.ed25519.0,
        id: inner_value.id,
    };

    let strkey = MuxedAccountEd25519(muxed_account);

    let str_result = format!("{strkey}");

    str_result
}

pub fn extract_base_address(address: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = PublicKey::from_string(address);

    if key.is_ok() {
        return Ok(address.to_string());
    }

    let key = MuxedAccount::from_string(address);
    if key.is_err() {
        return Err(format!("expected muxed account (M...), got {}", address).into());
    }
    let muxed_account = decode_address_to_muxed_account(address); // Replace with your actual decoding function
    let ed25519_key = muxed_account.ed25519;
    let encoded_ed25519 = PublicKey::from_payload(&ed25519_key).unwrap();
    Ok(encoded_ed25519.to_string())
}
