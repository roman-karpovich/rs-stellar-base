use std::str::FromStr;
use arrayref::array_ref;
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use stellar_xdr::*;

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