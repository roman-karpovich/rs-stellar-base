use stellar_xdr::*;
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use std::str::FromStr;

pub fn decode_address_to_muxed_account(address: &str) -> MuxedAccount{
    let muxed_key = MuxedAccount::from_string(address).unwrap();
    return muxed_key
}

//TODO: Encode MuxedAccount