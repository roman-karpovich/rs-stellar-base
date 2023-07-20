use std::str::FromStr;
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use stellar_xdr::*;

/// Create MuxedAccount obj from string
pub fn decode_address_to_muxed_account(address: &str) -> MuxedAccount {
    MuxedAccount::from_string(address).unwrap()
}

//TODO: Encode MuxedAccount
