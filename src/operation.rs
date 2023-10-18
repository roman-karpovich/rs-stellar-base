//! Operations are individual commands that modify the ledger.
use hex_literal::hex;
use num_bigint::BigInt;
use num_traits::identities::One;
use num_traits::{FromPrimitive, Num, Signed, Zero};
use stellar_strkey::ed25519::{MuxedAccount, PublicKey};
use std::collections::HashMap;
use std::str::FromStr;
use stellar_xdr::next::Type::Int64;
use stellar_xdr::next::{WriteXdr, AccountId};
use num_bigint::BigUint;
use num_traits::ToPrimitive;


use crate::asset::Asset;
use crate::liquidity_pool_asset::LiquidityPoolAsset;
use crate::utils::decode_encode_muxed_account::{decode_address_to_muxed_account, encode_muxed_account_to_address};

const ONE: i32 = 10_000_000;
const MAX_INT64: &str = "9223372036854775807";


pub const AUTH_REQUIRED_FLAG: u32 = 1 << 0;
pub const AUTH_REVOCABLE_FLAG: u32 = 1 << 1;
pub const AUTH_IMMUTABLE_FLAG: u32 = 1 << 2;

pub struct Operation;

pub struct OpAttributes {
    source_account: MuxedAccount,
}
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

pub struct Opts {
    source: Option<String>,
}

impl Operation {
    pub fn set_source_account(op_attributes: &mut OpAttributes, opts: &Opts) {
        if let Some(source) = &opts.source {
            match decode_address_to_muxed_account(source) {
                muxed_account => op_attributes.source_account = muxed_account,
                _ => panic!("Source address is invalid"),
            }
        }
    }

    pub fn from_xdr_object(operation: stellar_xdr::next::Operation) -> Result<HashMap<String, Value>, &'static str> {
        let mut result: HashMap<String, Value> = HashMap::new();
    
        if let Some(source_account) = operation.source_account {
            result.insert("source".to_string(), Value::Single(encode_muxed_account_to_address(&source_account)));
        }


        match operation.body {
            stellar_xdr::next::OperationBody::CreateAccount(x) => {
                result.insert("type".to_string(), Value::Single("createAccount".to_string()));
                result.insert("destination".to_string(), Value::Single(account_id_to_address(&x.destination)));
                result.insert("startingBalance".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.starting_balance as u64)).to_string()));
            },
            stellar_xdr::next::OperationBody::Payment(x) => {
                result.insert("type".to_string(), Value::Single("payment".to_string()));
                result.insert("destination".to_string(), Value::Single(encode_muxed_account_to_address(&x.destination)));
                result.insert("asset".to_string(), Value::Single(Asset::from_operation(x.asset).unwrap().to_string()));
                result.insert("amount".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.amount as u64)).to_string()));
            },
            stellar_xdr::next::OperationBody::PathPaymentStrictReceive(x) => {
                result.insert("type".to_string() ,Value::Single("pathPaymentStrictReceive".to_string()));
                result.insert("sendAsset".to_string(), Value::Single(Asset::from_operation(x.send_asset).unwrap().to_string()));
                result.insert("sendMax".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.send_max as u64)).to_string()));
                result.insert("destination".to_string(), Value::Single(encode_muxed_account_to_address(&x.destination)));
                result.insert("destAsset".to_string(), Value::Single(Asset::from_operation(x.dest_asset).unwrap().to_string()));
                result.insert("destAmount".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.dest_amount as u64)).to_string()));
                let mut path_vec = Vec::new();
                for path_key in x.path.iter() {
                    path_vec.push(Asset::from_operation(path_key.clone()).unwrap().to_string());
                }
                result.insert("path".to_string(), Value::Multiple(path_vec));
            },
            stellar_xdr::next::OperationBody::ManageSellOffer(_) => todo!(),
            stellar_xdr::next::OperationBody::CreatePassiveSellOffer(_) => todo!(),
            stellar_xdr::next::OperationBody::SetOptions(_) => todo!(),
            stellar_xdr::next::OperationBody::ChangeTrust(x) =>{
                result.insert("type".to_string(), Value::Single("changeTrust".to_string()));
                match x.line {
                    stellar_xdr::next::ChangeTrustAsset::Native => {
                        result.insert("line".to_string(), Value::Single(Asset::native().to_string()));
                    },
                    stellar_xdr::next::ChangeTrustAsset::CreditAlphanum4(x) => {
                        result.insert("line".to_string(), Value::Single(LiquidityPoolAsset::from_operation( &stellar_xdr::next::ChangeTrustAsset::CreditAlphanum4(x)).unwrap().to_string()));
                    },
                    stellar_xdr::next::ChangeTrustAsset::CreditAlphanum12(x) => {
                        result.insert("line".to_string(), Value::Single(LiquidityPoolAsset::from_operation( &stellar_xdr::next::ChangeTrustAsset::CreditAlphanum12(x)).unwrap().to_string()));
                    },
                    stellar_xdr::next::ChangeTrustAsset::PoolShare(x) => {
                        result.insert("line".to_string(), Value::Single(LiquidityPoolAsset::from_operation(&stellar_xdr::next::ChangeTrustAsset::PoolShare(x) ).unwrap().to_string()));
                    },
                
                }
                result.insert("limit".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.limit as u64)).to_string()));
            },
            stellar_xdr::next::OperationBody::AllowTrust(x) => {
                result.insert("type".to_string(), Value::Single("allowTrust".to_string()));
                result.insert("trustor".to_string(), Value::Single(account_id_to_address(&x.trustor)));
                let asset_code = match x.asset {
                    stellar_xdr::next::AssetCode::CreditAlphanum4(x) => x.to_string().trim_end_matches('\0').to_string(),
                    stellar_xdr::next::AssetCode::CreditAlphanum12(x) => x.to_string().trim_end_matches('\0').to_string(),
                };
                result.insert("assetCode".to_string(), Value::Single(asset_code));
                result.insert("authorize".to_string(), Value::Single(x.authorize.to_string()));
            },
            stellar_xdr::next::OperationBody::AccountMerge(_) => todo!(),
            stellar_xdr::next::OperationBody::Inflation => todo!(),
            stellar_xdr::next::OperationBody::ManageData(_) => todo!(),
            stellar_xdr::next::OperationBody::BumpSequence(_) => todo!(),
            stellar_xdr::next::OperationBody::ManageBuyOffer(_) => todo!(),
            stellar_xdr::next::OperationBody::PathPaymentStrictSend(x) => {
                result.insert("type".to_string(), Value::Single("pathPaymentStrictSend".to_string()));
                result.insert("sendAsset".to_string(), Value::Single(Asset::from_operation(x.send_asset).unwrap().to_string()));
                result.insert("sendAmount".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.send_amount as u64)).to_string()));
                result.insert("destination".to_string(), Value::Single(encode_muxed_account_to_address(&x.destination)));
                result.insert("destAsset".to_string(), Value::Single(Asset::from_operation(x.dest_asset).unwrap().to_string()));
                result.insert("destMin".to_string(), Value::Single(from_xdr_amount(BigUint::from(x.dest_min as u64)).to_string()));
                let mut path_vec = Vec::new();
                for path_key in x.path.iter() {
                    path_vec.push(Asset::from_operation(path_key.clone()).unwrap().to_string());
                }
                result.insert("path".to_string(), Value::Multiple(path_vec));
            },
            stellar_xdr::next::OperationBody::CreateClaimableBalance(_) => todo!(),
            stellar_xdr::next::OperationBody::ClaimClaimableBalance(_) => todo!(),
            stellar_xdr::next::OperationBody::BeginSponsoringFutureReserves(_) => todo!(),
            stellar_xdr::next::OperationBody::EndSponsoringFutureReserves => todo!(),
            stellar_xdr::next::OperationBody::RevokeSponsorship(_) => todo!(),
            stellar_xdr::next::OperationBody::Clawback(_) => todo!(),
            stellar_xdr::next::OperationBody::ClawbackClaimableBalance(_) => todo!(),
            stellar_xdr::next::OperationBody::SetTrustLineFlags(_) => todo!(),
            stellar_xdr::next::OperationBody::LiquidityPoolDeposit(_) => todo!(),
            stellar_xdr::next::OperationBody::LiquidityPoolWithdraw(_) => todo!(),
            stellar_xdr::next::OperationBody::InvokeHostFunction(_) => todo!(),
            stellar_xdr::next::OperationBody::BumpFootprintExpiration(_) => todo!(),
            stellar_xdr::next::OperationBody::RestoreFootprint(_) => todo!(),
            // "createAccount" => {
            //     result.insert("type", "createAccount");
            //     result.insert("destination", account_id_to_address(attrs.destination()));
            //     result.insert("startingBalance", self.from_xdr_amount(attrs.starting_balance()));
            // },
            // "payment" => {
            //     result.insert("type", "payment");
            //     result.insert("destination", encode_muxed_account_to_address(attrs.destination()));
            //     result.insert("asset", Asset::from_operation(attrs.asset()));
            //     result.insert("amount", self.from_xdr_amount(attrs.amount()));
            // },
            // "pathPaymentStrictReceive" => {
            //     result.insert("type", "pathPaymentStrictReceive");
            //     result.insert("sendAsset", Asset::from_operation(attrs.send_asset()));
            //     result.insert("sendMax", self.from_xdr_amount(attrs.send_max()));
            //     result.insert("destination", encode_muxed_account_to_address(attrs.destination()));
            //     result.insert("destAsset", Asset::from_operation(attrs.dest_asset()));
            //     result.insert("destAmount", self.from_xdr_amount(attrs.dest_amount()));
            //     let mut path_vec = Vec::new();
            //     for path_key in attrs.path().keys() {
            //         path_vec.push(Asset::from_operation(attrs.path()[&path_key]));
            //     }
            //     result.insert("path", path_vec);
            // },
            // "pathPaymentStrictSend" => {
            //     result.insert("type", "pathPaymentStrictSend");
            //     result.insert("sendAsset", Asset::from_operation(attrs.send_asset()));
            //     result.insert("sendAmount", self.from_xdr_amount(attrs.send_amount()));
            //     result.insert("destination", encode_muxed_account_to_address(attrs.destination()));
            //     result.insert("destAsset", Asset::from_operation(attrs.dest_asset()));
            //     result.insert("destMin", self.from_xdr_amount(attrs.dest_min()));
            //     let mut path_vec = Vec::new();
            //     for path_key in attrs.path().keys() {
            //         path_vec.push(Asset::from_operation(attrs.path()[&path_key]));
            //     }
            //     result.insert("path", path_vec);
            // },
            // "changeTrust" => {
            //     result.insert("type", "changeTrust");
            //     match attrs.line().switch() {
            //         xdr::AssetType::AssetTypePoolShare => {
            //             result.insert("line", LiquidityPoolAsset::from_operation(attrs.line()));
            //         },
            //         _ => {
            //             result.insert("line", Asset::from_operation(attrs.line()));
            //         }
            //     }
            //     result.insert("limit", self.from_xdr_amount(attrs.limit()));
            // },
        
        }
        
        Ok(result)
    }
}
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
pub fn to_xdr_amount(value: &str) -> Result<stellar_xdr::next::Int64, Box<dyn std::error::Error>> {
    let amount = BigInt::from_str_radix(value, 10)?;
    let one = BigInt::one();
    let xdr_amount = amount * &one;
    let xdr_string = xdr_amount.to_string();
    let xdr_int64 = stellar_xdr::next::Int64::from_str(&xdr_string)?;
    Ok(xdr_int64)
}
pub fn from_xdr_amount(value: BigUint) -> f64 {
    // Convert the value to f64, divide by ONE, and keep up to 7 decimal places
    round_to((value.to_f64().unwrap() / ONE as f64), 7)
}


// Utility function to round an f64 to a specific number of decimal places
pub fn round_to(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

fn account_id_to_address(account_id: &AccountId) -> String {

    let val = match account_id.0.clone() {
        stellar_xdr::next::PublicKey::PublicKeyTypeEd25519(x) => x,
    };
    let key: Result<PublicKey, stellar_strkey::DecodeError> = PublicKey::from_string(val.to_string().as_str());

    if key.is_ok() {
        return val.to_string();
    } else {
        panic!("Invalid account");
    }
}

#[cfg(test)]
mod tests {

    use stellar_xdr::next::{Int64, Operation, OperationBody, ReadXdr};

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
