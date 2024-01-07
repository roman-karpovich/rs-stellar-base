use std::collections::hash_map::ValuesMut;
use std::error::Error;
use std::str::FromStr;

use hex_literal::hex;
use num_bigint::BigUint;
use stellar_xdr::next::DecoratedSignature;

use crate::account::Account;
use crate::hashing::hash;
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
}

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
            sequence: "0".to_string(),
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

    use sha2::digest::crypto_common::Key;

    use super::*;
    use crate::{account::Account, keypair::Keypair, network::Networks, transaction::TransactionBehavior};

    #[test]
    fn test_creates_and_signs() {
        let source = Account::new(
            "GBBM6BKZPEHWYO3E3YKREDPQXMS4VK35YLNU7NFBRI26RAN7GI5POFBB",
            "20",
        )
        .unwrap();
        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let signer = Keypair::master(Some(Networks::TESTNET)).unwrap();
        let mut tx = TransactionBuilder::new(source, Networks::TESTNET)
            .fee(100_u32)
            .add_operation(create_account(destination, "10".to_string()).unwrap())
            .build();

        tx.sign(&[signer.clone()]);
        let sig = &tx.signatures[0].signature.0;
        let verified = signer.verify(&tx.hash(), sig);
        assert_eq!(verified, true);
    }
}
