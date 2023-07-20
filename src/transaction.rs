use std::collections::hash_map::ValuesMut;
use std::error::Error;
use std::str::FromStr;

use num_bigint::BigUint;
use stellar_xdr::DecoratedSignature;
use hex_literal::hex;

use stellar_xdr::Memo;
use stellar_xdr::MuxedAccount;
use stellar_xdr::Operation;
use stellar_xdr::Preconditions;
use stellar_xdr::ReadXdr;
use stellar_xdr::SequenceNumber;
use stellar_xdr::Signature;
use stellar_xdr::TimeBounds;
use stellar_xdr::LedgerBounds;
use stellar_xdr::TransactionEnvelope;
use stellar_xdr::TransactionExt;
use stellar_xdr::TransactionV1Envelope;
use stellar_xdr::Uint256;
use stellar_xdr::VecM;
use stellar_xdr::WriteXdr;
use crate::account::Account;
use crate::hashing::hash;
use crate::keypair::Keypair;
use stellar_xdr::TransactionV0Envelope;
use crate::op_list::create_account::create_account;

#[derive(Debug)]
pub struct Transaction {
    tx: Option<stellar_xdr::Transaction>,
    network_passphrase: String,
    signatures: Vec<DecoratedSignature>,
    fee: u32,
    envelope_type: stellar_xdr::EnvelopeType,
    memo: Option<stellar_xdr::Memo>,
    sequence: String,
    source: String,
    time_bounds: Option<TimeBounds>,
    ledger_bounds: Option<LedgerBounds>,
    min_account_sequence: Option<String>,
    min_account_sequence_age: u32,
    min_account_sequence_ledger_gap: u32,
    extra_signers: Vec<stellar_xdr::AccountId>,
    operations: Option<Vec<Operation>>,
    hash: Option<[u8; 32]>,
}

impl Transaction {
    fn signature_base(&self) -> Vec<u8> {
        let mut tx = self.tx.clone().unwrap().clone();
        let tagged_tx =stellar_xdr::TransactionSignaturePayloadTaggedTransaction::Tx(tx);
        let tx_sig = stellar_xdr::TransactionSignaturePayload {
            network_id: stellar_xdr::Hash(hash(self.network_passphrase.as_str())),
            tagged_transaction: tagged_tx,
        };
        tx_sig.to_xdr().unwrap()
    }

    fn hash(&self) ->  [u8; 32]{
        hash(&self.signature_base())
    }

    fn sign(&mut self, keypairs: &[Keypair]) {
        let tx_hash: [u8; 32] = self.hash();
        for kp in keypairs {
            let sig = kp.sign_decorated(&tx_hash);
            self.signatures.push(sig);
        }

        self.hash = Some(tx_hash);
    }

    fn to_envelope(&mut self) -> Result<TransactionEnvelope, Box<dyn Error>> {
        let raw_tx = self.tx.to_xdr().unwrap();
        let mut signatures = VecM::<DecoratedSignature, 20>::try_from(self.signatures.clone()).unwrap(); // Make a copy of the signatures
        let envelope = match self.envelope_type {
            stellar_xdr::EnvelopeType::TxV0 => {
                let transaction_v0 = stellar_xdr::TransactionV0Envelope {
                    tx: stellar_xdr::TransactionV0::from_xdr(&raw_tx).unwrap(), // Make a copy of tx
                    signatures,
                };
                stellar_xdr::TransactionEnvelope::TxV0(transaction_v0)
            }
            stellar_xdr::EnvelopeType::Tx => {
                let transaction_v1 = stellar_xdr::TransactionV1Envelope {
                    tx: stellar_xdr::Transaction::from_xdr(&raw_tx).unwrap(), // Make a copy of tx
                    signatures,
                };
                stellar_xdr::TransactionEnvelope::Tx(transaction_v1)
            }
            _ => {
                return Err(format!(
                    "Invalid TransactionEnvelope: expected an envelopeTypeTxV0 or envelopeTypeTx but received an {:?}.",
                    self.envelope_type
                )
                .into());
            }
        };
    
        Ok(envelope)
    }
}

#[derive(Default)]
pub struct TransactionBuilder {
    tx: Option<stellar_xdr::Transaction>,
    network_passphrase: Option<String>,
    signatures: Option<Vec<DecoratedSignature>>,
    fee: Option<u32>,
    envelope_type: Option<stellar_xdr::EnvelopeType>,
    memo: Option<stellar_xdr::Memo>,
    sequence: Option<String>,
    source: Option<Account>,
    time_bounds: Option<TimeBounds>,
    ledger_bounds: Option<LedgerBounds>,
    min_account_sequence: Option<String>,
    min_account_sequence_age: Option<u32>,
    min_account_sequence_ledger_gap: Option<u32>,
    extra_signers: Option<Vec<stellar_xdr::AccountId>>,
    operations: Option<Vec<Operation>>
}

impl TransactionBuilder {
    pub fn new(source_account: Account, network: &str) -> Self {

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
            operations: Some(Vec::new())
        }
       
	}

    pub fn fee(&mut self, fee: impl Into<u32>) -> &mut Self {
		self.fee.insert(fee.into());
		self
	}

    pub fn add_operation(&mut self, operation: Operation) -> &mut Self {
        if let Some(ref mut vec) = self.operations {
            vec.push(operation);
        }
        self
    }

    pub fn build(&mut self ) -> Transaction {
        let seq_num = BigUint::from_str(self.source.clone().unwrap().sequence_number().as_str()).unwrap();
        let fee = self.fee.unwrap().checked_mul(self.operations.clone().unwrap().len().try_into().unwrap());
        let val = self.source.clone().unwrap();
        let vv = val.clone();
        let vv2 = vv.account_id();
        let binding = hex::encode(vv2).clone();
        let hex_val = binding.as_bytes().clone();
        let mut array: [u8; 32] = [0; 32];
        array.copy_from_slice(&hex_val[..32]);

        let tx_obj = stellar_xdr::Transaction {
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
            envelope_type: stellar_xdr::EnvelopeType::Tx,
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

    use crate::{account::Account, keypair::Keypair, network::Networks};
    use super::*;

    #[test]
    fn test_creates_and_signs() {
        let source = Account::new("GBBM6BKZPEHWYO3E3YKREDPQXMS4VK35YLNU7NFBRI26RAN7GI5POFBB", "20").unwrap();
        let destination = "GDJJRRMBK4IWLEPJGIE6SXD2LP7REGZODU7WDC3I2D6MR37F4XSHBKX2".to_string();
        let signer = Keypair::master(Some(Networks::TESTNET)).unwrap();
        let mut tx = TransactionBuilder::new(source, Networks::TESTNET)
            .fee(100_u32)
            .add_operation(create_account(destination, "10".to_string()).unwrap())
            .build();

        tx.sign(&[signer.clone()]);
        let sig = &tx.signatures[0].signature.0;
        let verified = signer.verify(&tx.hash(),sig);
        assert_eq!(verified, true);
    }
    
}