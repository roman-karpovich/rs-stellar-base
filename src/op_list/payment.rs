use xdr;
use crate::{util::decode_encode_muxed_account::{decode_address_to_muxed_account}, operation::{is_valid_amount, to_xdr_amount}, asset::Asset};
use std::option::Option;

/// Create a payment operation.
///
/// See https://developers.stellar.org/docs/start/list-of-operations/#payment
///
/// # Arguments
///
/// * `destination` - Destination account ID.
/// * `asset` - Asset to send.
/// * `amount` - Amount to send.
/// * `source` - (Optional) The source account for the payment. Defaults to the transaction's source account.
///
/// # Returns
///
/// * A Result containing the resulting payment operation (`xdr::PaymentOp`) or an error.
pub fn payment(destination: &str, asset: Asset, amount: &str, source: Option<&str>) -> Result<xdr::Operation, &'static str> {
    
    if !is_valid_amount(amount, false) {
        return panic!("Error: Invalid amount")
    }

    let mut attributes = PaymentAttributes::default();

    match decode_address_to_muxed_account(destination) {
        Ok(muxed_account) => attributes.destination = muxed_account,
        Err(_) => return Err("destination is invalid"),
    }

    attributes.asset = asset.to_xdr_object();
    attributes.amount = to_xdr_amount(amount);

    let payment_op = xdr::PaymentOp::new(attributes);

    let mut op_attributes = OperationAttributes::default();
    op_attributes.body = xdr::OperationBody::Payment(payment_op);
    set_source_account(&mut op_attributes, source);

    Ok(xdr::Operation::new(op_attributes))
}

// Placeholder structures and functions for the above code. Actual implementations would be required.

#[derive(Default)]
struct PaymentAttributes {
    destination: MuxedAccount,
    asset: xdr::Asset,
    amount: xdr::XdrAmount,
}

#[derive(Default)]
struct OperationAttributes {
    body: xdr::OperationBody,
}

// struct Asset {
//     // ... asset fields here ...
// }

// impl Asset {
//     pub fn to_xdr_object(&self) -> xdr::Asset {
//         // ... convert self to xdr::Asset ...
//         unimplemented!()
//     }
// }

// struct MuxedAccount;
// type XdrAmount = i64;

// impl Self {
//     fn is_valid_amount(amount: &str) -> bool {
//         // ... check if amount is valid ...
//         unimplemented!()
//     }

//     fn construct_amount_requirements_error(field: &str) -> &'static str {
//         // ... construct error message ...
//         unimplemented!()
//     }

//     fn to_xdr_amount(amount: &str) -> XdrAmount {
//         // ... convert amount string to XdrAmount ...
//         unimplemented!()
//     }

//     fn set_source_account(op_attributes: &mut OperationAttributes, source: Option<&str>) {
//         // ... set source account on op_attributes ...
//         unimplemented!()
//     }
// }
