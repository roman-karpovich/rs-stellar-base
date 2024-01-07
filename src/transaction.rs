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

#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx: Option<stellar_xdr::next::Transaction>,
    pub network_passphrase: String,
    pub signatures: Vec<DecoratedSignature>,
    pub fee: u32,
    pub envelope_type: stellar_xdr::next::EnvelopeType,
    pub memo: Option<stellar_xdr::next::Memo>,
    pub sequence: String,
    pub source: String,
    pub time_bounds: Option<TimeBounds>,
    pub ledger_bounds: Option<LedgerBounds>,
    pub min_account_sequence: Option<String>,
    pub min_account_sequence_age: u32,
    pub min_account_sequence_ledger_gap: u32,
    pub extra_signers: Vec<stellar_xdr::next::AccountId>,
    pub operations: Option<Vec<Operation>>,
    pub hash: Option<[u8; 32]>,
}

// Define a trait for Transaction behavior
pub trait TransactionBehavior {
    fn signature_base(&self) -> Vec<u8>;
    fn hash(&self) -> [u8; 32];
    fn sign(&mut self, keypairs: &[Keypair]);
    fn to_envelope(&self) -> Result<TransactionEnvelope, Box<dyn Error>>;
}

impl TransactionBehavior for Transaction {
    fn signature_base(&self) -> Vec<u8> {
        let mut tx = self.tx.clone().unwrap();
        let tagged_tx = stellar_xdr::next::TransactionSignaturePayloadTaggedTransaction::Tx(tx);
        let tx_sig = stellar_xdr::next::TransactionSignaturePayload {
            network_id: stellar_xdr::next::Hash(hash(self.network_passphrase.as_str())),
            tagged_transaction: tagged_tx,
        };
        tx_sig.to_xdr(stellar_xdr::next::Limits::none()).unwrap()
    }

    fn hash(&self) -> [u8; 32] {
        hash(self.signature_base())
    }

    fn sign(&mut self, keypairs: &[Keypair]) {
        let tx_hash: [u8; 32] = self.hash();
        for kp in keypairs {
            let sig = kp.sign_decorated(&tx_hash);
            self.signatures.push(sig);
        }

        self.hash = Some(tx_hash);
    }

    fn to_envelope(&self) -> Result<TransactionEnvelope, Box<dyn Error>> {
        let raw_tx = self.tx.to_xdr(stellar_xdr::next::Limits::none()).unwrap();
        let mut signatures =
            VecM::<DecoratedSignature, 20>::try_from(self.signatures.clone()).unwrap(); // Make a copy of the signatures
        let envelope = match self.envelope_type {
            stellar_xdr::next::EnvelopeType::TxV0 => {
                let transaction_v0 = stellar_xdr::next::TransactionV0Envelope {
                    tx: stellar_xdr::next::TransactionV0::from_xdr(&raw_tx, stellar_xdr::next::Limits::none()).unwrap(), // Make a copy of tx
                    signatures,
                };
                stellar_xdr::next::TransactionEnvelope::TxV0(transaction_v0)
            }
            stellar_xdr::next::EnvelopeType::Tx => {
                let transaction_v1 = stellar_xdr::next::TransactionV1Envelope {
                    tx: stellar_xdr::next::Transaction::from_xdr(&raw_tx,stellar_xdr::next::Limits::none()).unwrap(), // Make a copy of tx
                    signatures,
                };
                stellar_xdr::next::TransactionEnvelope::Tx(transaction_v1)
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