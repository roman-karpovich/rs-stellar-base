use crate::{keypair::*, operation::to_xdr_amount, utils::decode_encode_muxed_account::decode_address_to_muxed_account};
use sha2::digest::crypto_common::Key;
use stellar_strkey::*;
use stellar_xdr::{*};
use stellar_strkey::ed25519::PublicKey;
use crate::operation::is_valid_amount;
use stellar_xdr::MuxedAccount;
use hex_literal::hex;
pub fn create_account(destination: String, starting_balance: String, source: String) -> Result<Operation, Box<dyn std::error::Error>> {
    
    let key = PublicKey::from_string(&destination);

    if key.is_err() {
        return Err("destination is invalid".into());
    }

    if !is_valid_amount(&starting_balance, true) {
        return Err("startingBalance must be of type String, represent a non-negative number and have at most 7 digits after the decimal".into());
    }
    let dest = Keypair::from_public_key(&destination).unwrap().xdr_account_id();
    let starting_balance = to_xdr_amount(&starting_balance)?;
    let body  = stellar_xdr::OperationBody::CreateAccount(
        CreateAccountOp { destination: dest, starting_balance: starting_balance }
    );

    Ok(stellar_xdr::Operation {source_account: None, body})
}