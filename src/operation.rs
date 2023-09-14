//! Operations are individual commands that modify the ledger.
use hex_literal::hex;
use num_bigint::BigInt;
use num_traits::identities::One;
use num_traits::{FromPrimitive, Num, Signed, Zero};
use std::str::FromStr;
use stellar_xdr::curr::Type::Int64;
use stellar_xdr::curr::WriteXdr;

/// Validates that a given amount is possible for a Stellar asset.
pub fn is_valid_amount(value: &str, allow_zero: bool) -> bool {
    if !value.is_empty() {
        if let Ok(amount) = BigInt::from_str_radix(value, 10) {
            if !allow_zero && amount.is_zero() {
                return false;
            }

            let max_int64: BigInt = FromPrimitive::from_i64(i64::MAX).unwrap();
            let one = BigInt::one();

            if amount.is_negative()
                || amount > max_int64
                || amount.to_string().chars().filter(|&c| c == '.').count() > 1
                || amount
                    .to_string()
                    .chars()
                    .skip_while(|&c| c != '.')
                    .skip(1)
                    .count()
                    > 7
            //TODO: Add case for checking infinite number and NaN
            {
                return false;
            }

            return true;
        }
    }

    false
}

/// xdr representation of the amount value
pub fn to_xdr_amount(value: &str) -> Result<stellar_xdr::curr::Int64, Box<dyn std::error::Error>> {
    let amount = BigInt::from_str_radix(value, 10)?;
    let one = BigInt::one();
    let xdr_amount = amount * &one;
    let xdr_string = xdr_amount.to_string();
    let xdr_int64 = stellar_xdr::curr::Int64::from_str(&xdr_string)?;
    Ok(xdr_int64)
}

#[cfg(test)]
mod tests {

    use stellar_xdr::curr::{Int64, Operation, OperationBody, ReadXdr};

    use crate::{account::Account, keypair::Keypair, op_list::create_account::create_account};

    use super::*;

    #[test]
    fn create_account_op_test() {
        let destination = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ".to_string();
        let destination_hex =
            hex!("899b2840ed5636c56ddc5f14b23975f79f1ba2388d2694e4c56ecdddc960e5ef");
        // println!("Destination hex {:?}", destination_hex);
        let starting_balance = "1000".to_string();

        let op = create_account(destination.clone(), starting_balance).unwrap();

        let op = Operation::to_xdr(&op).unwrap();
        let op_from = Operation::from_xdr(op.as_slice()).unwrap().body;

        if let OperationBody::CreateAccount(op) = &op_from {
            assert_eq!(op.starting_balance, 1000);
            let mut result: [u8; 32] = Default::default();
            result[..32].clone_from_slice(&destination_hex);
            let key = Keypair::new(Some(result), None).unwrap();
            let val = key.xdr_public_key();
            assert_eq!(op.destination.0, val);
        } else {
            panic!("op is not the type expected");
        }
    }
}
