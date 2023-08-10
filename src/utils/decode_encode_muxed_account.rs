use std::str::FromStr;
use arrayref::array_ref;
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use stellar_xdr::*;
use stellar_strkey::Strkey::MuxedAccountEd25519;

use crate::muxed_account;

/// Create MuxedAccount obj from string
pub fn decode_address_to_muxed_account(address: &str) -> MuxedAccount {
    MuxedAccount::from_string(address).unwrap()
}

pub fn encode_muxed_account(address: &str, id: &str) -> stellar_xdr::MuxedAccount {
  
    let key = PublicKey::from_string(address);
   
    if key.is_err() {
        panic!("address should be a Stellar account ID (G...)");
    }
    if !id.parse::<u64>().is_ok() {
        panic!("id should be a string representing a number (uint64)");
    }
    let binding = key.unwrap().clone().to_string();
    let val = binding.as_bytes();
    stellar_xdr::MuxedAccount::MuxedEd25519(
        stellar_xdr::MuxedAccountMed25519 {
            id: id.parse::<u64>().unwrap(),
            ed25519: Uint256(*array_ref!(val, 0, 32))
        }
    )
}

pub fn encode_muxed_account_to_address(muxed_account: &stellar_xdr::MuxedAccount) -> String {

    if muxed_account.discriminant() == stellar_xdr::CryptoKeyType::MuxedEd25519 {
        return _encode_muxed_account_fully_to_address(muxed_account);
    }

    let inner_value = match muxed_account {
        stellar_xdr::MuxedAccount::Ed25519(inner) => inner,
        _ => panic!("Expected Ed25519 variant"),
    };

   let key = PublicKey::from_payload(&inner_value.0).unwrap().to_string();

   key
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

    muxed_account.to_string()
    
}