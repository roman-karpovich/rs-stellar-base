use num_bigint::BigInt;
use num_traits::{Zero, FromPrimitive, Num, Signed};
use num_traits::identities::One;
use std::str::FromStr;

pub(crate) fn is_valid_amount(value: &str, allow_zero: bool) -> bool {
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
                || amount.to_string().chars().skip_while(|&c| c != '.').skip(1).count() > 7
                //TODO: Add case for checking infinite number and NaN
            {
                return false;
            }

            return true;
        }
    }

    false
}



pub fn to_xdr_amount(value: &str) -> Result<stellar_xdr::Int64, Box<dyn std::error::Error>> {
    let amount = BigInt::from_str_radix(value, 10)?;
    let one = BigInt::one();
    let xdr_amount = amount * &one;
    let xdr_string = xdr_amount.to_string();
    let xdr_int64 = stellar_xdr::Int64::from_str(&xdr_string)?;
    Ok(xdr_int64)
}