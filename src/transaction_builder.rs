use std::collections::hash_map::ValuesMut;
use std::error::Error;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use hex_literal::hex;
use num_bigint::BigUint;
use serde_json::from_str;
use stellar_xdr::next::DecoratedSignature;
use stellar_xdr::next::StringM;

use crate::account::Account;
use crate::account::AccountBehavior;
use crate::hashing::Sha256Hasher;
use crate::keypair::Keypair;
use crate::op_list::create_account::create_account;
use crate::transaction::Transaction;
use stellar_xdr::next::LedgerBounds;
use stellar_xdr::next::Memo;
use stellar_xdr::next::MuxedAccount;
use stellar_xdr::next::Operation;
use stellar_xdr::next::Preconditions;
use stellar_xdr::next::ReadXdr;
use stellar_xdr::next::SequenceNumber;
use stellar_xdr::next::Signature;
use stellar_xdr::next::TimeBounds;
use stellar_xdr::next::TransactionEnvelope;
use stellar_xdr::next::TransactionExt;
use stellar_xdr::next::TransactionV0Envelope;
use stellar_xdr::next::TransactionV1Envelope;
use stellar_xdr::next::Uint256;
use stellar_xdr::next::VecM;
use stellar_xdr::next::WriteXdr;

#[derive(Default, Clone)]
pub struct TransactionBuilder {
    tx: Option<stellar_xdr::next::Transaction>,
    network_passphrase: Option<String>,
    signatures: Option<Vec<DecoratedSignature>>,
    fee: Option<u32>,
    envelope_type: Option<stellar_xdr::next::EnvelopeType>,
    memo: Option<stellar_xdr::next::Memo>,
    sequence: Option<String>,
    source: Option<Account>,
    time_bounds: Option<TimeBounds>,
    ledger_bounds: Option<LedgerBounds>,
    min_account_sequence: Option<String>,
    min_account_sequence_age: Option<u32>,
    min_account_sequence_ledger_gap: Option<u32>,
    extra_signers: Option<Vec<stellar_xdr::next::AccountId>>,
    operations: Option<Vec<Operation>>,
}

// Define a trait for TransactionBuilder behavior
pub trait TransactionBuilderBehavior {
    fn new(source_account: Account, network: &str) -> Self;
    fn fee(&mut self, fee: impl Into<u32>) -> &mut Self;
    fn add_operation(&mut self, operation: Operation) -> &mut Self;
    fn build(&mut self) -> Transaction;
    fn add_memo(&mut self, memo_text: &str) -> &mut Self;
    fn set_timeout(&mut self, timeout_seconds: i64) -> Result<&mut Self, String>;
}

pub const TIMEOUT_INFINITE: i64 = 0;

impl TransactionBuilderBehavior for TransactionBuilder {
    fn new(source_account: Account, network: &str) -> Self {
        Self {
            tx: None,
            network_passphrase: Some(network.to_string()),
            signatures: None,
            fee: None,
            envelope_type: None,
            memo: None,
            sequence: None,
            source: Some(source_account),
            time_bounds: None,
            ledger_bounds: None,
            min_account_sequence: None,
            min_account_sequence_age: None,
            min_account_sequence_ledger_gap: None,
            extra_signers: None,
            operations: Some(Vec::new()),
        }
    }

    fn fee(&mut self, fee: impl Into<u32>) -> &mut Self {
        self.fee.insert(fee.into());
        self
    }

    fn add_operation(&mut self, operation: Operation) -> &mut Self {
        if let Some(ref mut vec) = self.operations {
            vec.push(operation);
        }
        self
    }

    fn add_memo(&mut self, memo_text: &str) -> &mut Self {
        self.memo = Some(stellar_xdr::next::Memo::Text(StringM::<28>::from_str(memo_text).unwrap()));
        self
    }

    fn set_timeout(&mut self, timeout_seconds: i64) -> Result<&mut Self, String> {
        if let Some(timebounds) = &self.time_bounds {
            if timebounds.max_time > stellar_xdr::next::TimePoint(0) {
                return Err("TimeBounds.max_time has been already set - setting timeout would overwrite it.".to_string());
            }
        }

        if timeout_seconds < 0 {
            return Err("timeout cannot be negative".to_string());
        }

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Error getting current time: {}", e))?
            .as_secs();

        if timeout_seconds > 0 {
            let timeout_timestamp = current_time + timeout_seconds as u64;
            self.time_bounds = Some(TimeBounds {
                min_time: self.time_bounds.as_ref().map_or(stellar_xdr::next::TimePoint(0), |tb| tb.min_time.clone()),
                max_time: stellar_xdr::next::TimePoint(timeout_timestamp),
            });
        } else {
            self.time_bounds = Some(TimeBounds {
                min_time: stellar_xdr::next::TimePoint(0),
                max_time: stellar_xdr::next::TimePoint(0),
            });
        }

        Ok(self)
    }

    fn build(&mut self) -> Transaction {
        let seq_num =
            BigUint::from_str(self.source.clone().unwrap().sequence_number().as_str()).unwrap();
        let fee = self
            .fee
            .unwrap()
            .checked_mul(self.operations.clone().unwrap().len().try_into().unwrap());
        let val = self.source.clone().unwrap();
        let vv = val;
        let vv2 = vv.account_id();
        let binding = hex::encode(vv2);
        let hex_val = binding.as_bytes();
        let mut array: [u8; 32] = [0; 32];
        array.copy_from_slice(&hex_val[..32]);

        let tx_obj = stellar_xdr::next::Transaction {
            source_account: MuxedAccount::Ed25519(Uint256::from(array)), // MuxedAccount::Ed25519(Uint256([0; 32]))
            fee: fee.unwrap(),
            seq_num: SequenceNumber(1_i64),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: self.operations.clone().unwrap().try_into().unwrap(),
            ext: TransactionExt::V0,
        };
        Transaction {
            tx: Some(tx_obj),
            network_passphrase: self.network_passphrase.clone().unwrap(),
            signatures: Vec::new(),
            fee: fee.unwrap(),
            envelope_type: stellar_xdr::next::EnvelopeType::Tx,
            memo: None,
            sequence: "1".to_string(),
            source: self.source.clone().unwrap().account_id().to_string(),
            time_bounds: None,
            ledger_bounds: None,
            min_account_sequence: Some("0".to_string()),
            min_account_sequence_age: 0,
            min_account_sequence_ledger_gap: 0,
            extra_signers: Vec::new(),
            operations: self.operations.clone(),
            hash: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use core::panic;
    use keypair::KeypairBehavior;

    use sha2::digest::crypto_common::Key;

    use super::*;
    // use crate::{
    //     account::Account, asset::{Asset, AssetBehavior}, keypair::{self, Keypair}, network::{NetworkPassphrase, Networks}, operation::PaymentOpts, transaction::TransactionBehavior
    // };
    use crate::{
        account::{Account, AccountBehavior}, asset::{Asset,AssetBehavior}, keypair::{self, Keypair}, network::{NetworkPassphrase, Networks}, operation::{Operation, OperationBehavior, PaymentOpts}, transaction::TransactionBehavior, transaction_builder::{TransactionBuilder, TransactionBuilderBehavior, TIMEOUT_INFINITE}
    };

    

    #[test]
    fn test_creates_and_signs() {
        let source = Account::new(
            "GBBM6BKZPEHWYO3E3YKREDPQXMS4VK35YLNU7NFBRI26RAN7GI5POFBB",
            "20",
        )
        .unwrap();
        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let signer = Keypair::master(Some(Networks::testnet())).unwrap();
        let mut tx = TransactionBuilder::new(source, Networks::testnet())
            .fee(100_u32)
            .add_operation(create_account(destination, "10".to_string()).unwrap())
            .build();

        tx.sign(&[signer.clone()]);
        let sig = &tx.signatures[0].signature.0;
        let verified = signer.verify(&tx.hash(), sig);
        assert_eq!(verified, true);
    }

    #[test]
    fn test_constructs_native_payment_transaction() {
        let source = Account::new(
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
            "1",
        )
        .unwrap();

        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let amount = "1000".to_string();
        let asset = Asset::native(); 
        let memo = Memo::Id(100);
        let mut builder = TransactionBuilder::new(source.clone(), Networks::testnet());

        builder
            .fee(100_u32)
            .add_operation(Operation::payment(PaymentOpts {
                destination: destination.to_owned(),
                asset,
                amount: amount.to_owned(),
                source: None,
            }).unwrap())
            .add_memo("100")
            .set_timeout(TIMEOUT_INFINITE)
            .unwrap();

        let transaction = builder.build();

        assert_eq!(transaction.source, source.account_id().to_string());
        assert_eq!(transaction.sequence, "1");
        assert_eq!(source.sequence_number(), "1");
        assert_eq!(transaction.operations.unwrap().len(), 1);
        assert_eq!(transaction.fee, 100);
    }

}
